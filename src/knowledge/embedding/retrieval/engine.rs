//! Embedding-powered retrieval engine for knowledge search.

use crate::agent::runtime::metrics::RuntimeMetricsCollector;
use crate::knowledge::chunk::Chunk;
use crate::knowledge::document::Document;
use crate::knowledge::embedding::engine::{EmbeddingEngine, EmbeddingError, LocalEmbeddingEngine};
use crate::knowledge::embedding::model::{EmbeddingConfig};
use crate::knowledge::embedding::retrieval::{ContextBuilder, HybridRetrievalConfig, HybridRetrievalEngine, KeywordSearchEngine, SemanticSearchEngine};
use crate::knowledge::embedding::similarity::{CosineSimilarity, DotProductSimilarity, EuclideanDistanceSimilarity, SimilarityMetric, SimilarityMetricType};
use crate::knowledge::embedding::vector::{DenseVector, VectorIndex, VectorMetadata};
use crate::knowledge::embedding::vector::search::SearchResultSet;
use crate::knowledge::errors::KnowledgeResult;
use crate::knowledge::retrieval::{KnowledgeRetrievalEngine, RetrievalResult};
use crate::knowledge::traits::KnowledgeStorage;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Configuration for embedding-based knowledge retrieval.
#[derive(Debug, Clone)]
pub struct EmbeddingRetrievalConfig {
    pub embedding_dimension: usize,
    pub embedding_backend: String,
    pub top_k: usize,
    pub similarity_metric: SimilarityMetricType,
    pub semantic_weight: f32,
    pub keyword_weight: f32,
    pub min_score: f32,
    pub cache_enabled: bool,
    pub cache_directory: Option<String>,
    pub vector_directory: Option<String>,
}

impl Default for EmbeddingRetrievalConfig {
    fn default() -> Self {
        Self {
            embedding_dimension: 768,
            embedding_backend: "local".to_string(),
            top_k: 8,
            similarity_metric: SimilarityMetricType::Cosine,
            semantic_weight: 0.7,
            keyword_weight: 0.3,
            min_score: 0.1,
            cache_enabled: true,
            cache_directory: None,
            vector_directory: None,
        }
    }
}

impl EmbeddingRetrievalConfig {
    pub fn validate(&self) -> Result<(), String> {
        let sum = self.semantic_weight + self.keyword_weight;
        if (sum - 1.0).abs() > 0.01 {
            return Err(format!("hybrid weights must sum to 1.0, got {}", sum));
        }
        if self.top_k == 0 {
            return Err("top_k must be greater than 0".to_string());
        }
        if self.embedding_dimension == 0 {
            return Err("embedding_dimension must be greater than 0".to_string());
        }
        if self.similarity_metric == SimilarityMetricType::EuclideanDistance && self.min_score < 0.0 {
            return Err("min_score must be non-negative".to_string());
        }
        Ok(())
    }
}

/// Knowledge retrieval engine that combines embedding search with keyword search.
pub struct KnowledgeEmbeddingRetrievalEngine {
    config: EmbeddingRetrievalConfig,
    embedding_engine: Arc<dyn EmbeddingEngine>,
    vector_index: Arc<parking_lot::RwLock<VectorIndex>>,
    semantic_engine: SemanticSearchEngine,
    keyword_engine: Arc<KeywordSearchEngine>,
    hybrid_engine: HybridRetrievalEngine,
    chunk_map: Arc<parking_lot::RwLock<HashMap<String, Chunk>>>,
    metrics: Option<Arc<dyn RuntimeMetricsCollector>>,
}

impl KnowledgeEmbeddingRetrievalEngine {
    /// Create a new embedding retrieval engine from configuration.
    pub fn new(config: EmbeddingRetrievalConfig) -> Result<Self, String> {
        config.validate()?;

        let embedding_engine: Arc<dyn EmbeddingEngine> = match config.embedding_backend.as_str() {
            "local" => Arc::new(LocalEmbeddingEngine::new()),
            backend => {
                return Err(format!("unsupported embedding backend: {}", backend));
            }
        };

        if let Some(cache_dir) = &config.cache_directory {
            if config.cache_enabled {
                let engine = embedding_engine
                    .as_any()
                    .downcast_ref::<LocalEmbeddingEngine>()
                    .ok_or_else(|| "failed to downcast embedding engine".to_string())?;
                let _ = engine.load_cache(cache_dir);
            }
        }

        let similarity_metric: Arc<dyn SimilarityMetric> = match config.similarity_metric {
            SimilarityMetricType::Cosine => Arc::new(CosineSimilarity),
            SimilarityMetricType::DotProduct => Arc::new(DotProductSimilarity),
            SimilarityMetricType::EuclideanDistance => Arc::new(EuclideanDistanceSimilarity::default_temp()),
        };

        let vector_index = Arc::new(parking_lot::RwLock::new(VectorIndex::new(similarity_metric.clone())));
        let semantic_engine = SemanticSearchEngine::new(embedding_engine.clone(), vector_index.clone());
        let keyword_engine = Arc::new(KeywordSearchEngine::new()?);
        let hybrid_engine = HybridRetrievalEngine::new(HybridRetrievalConfig {
            semantic_weight: config.semantic_weight,
            keyword_weight: config.keyword_weight,
            min_score: config.min_score,
        })?;

        Ok(Self {
            config,
            embedding_engine,
            vector_index,
            semantic_engine,
            keyword_engine,
            hybrid_engine,
            chunk_map: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            metrics: None,
        })
    }

    /// Attach an optional runtime metrics collector for observability.
    pub fn with_metrics(mut self, metrics: Arc<dyn RuntimeMetricsCollector>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    fn ensure_index_loaded(&self, storage: &dyn KnowledgeStorage) -> Result<(), String> {
        if self.vector_index.read().is_empty()? {
            self.rebuild_index(storage)
        } else {
            Ok(())
        }
    }

    fn rebuild_index(&self, storage: &dyn KnowledgeStorage) -> Result<(), String> {
        let index_start = Instant::now();
        let documents = storage
            .list_documents()
            .map_err(|e| format!("storage error: {}", e))?;

        let mut entries = Vec::new();
        let mut chunk_map = self.chunk_map.write();
        chunk_map.clear();

        for document in documents.iter() {
            let chunks = storage
                .list_chunks(document.id())
                .map_err(|e| format!("storage error: {}", e))?;
            for chunk in chunks.into_iter() {
                let embedding = self
                    .embedding_engine
                    .embed_text(chunk.content())
                    .map_err(|e| format!("embedding failed: {}", e))?;

                let metadata = self.build_vector_metadata(document, &chunk);
                entries.push((chunk.id().to_string(), embedding, metadata));
                chunk_map.insert(chunk.id().to_string(), chunk.clone());
            }
        }

        {
            let mut index = self.vector_index.write();
            index.clear()?;
            index.batch_insert(entries)?;
        }

        self.keyword_engine.rebuild(storage)?;

        if let Some(metrics) = &self.metrics {
            metrics.record_duration(
                "embedding_index_build_duration",
                index_start.elapsed(),
                None,
            );
            metrics.incr("embedding_index_size", None);
        }

        Ok(())
    }

    fn build_vector_metadata(&self, document: &Document, chunk: &Chunk) -> VectorMetadata {
        VectorMetadata::new(
            chunk.id().to_string(),
            document.id().to_string(),
            chunk.sequence(),
            chunk.content().to_string(),
            document.language().to_string(),
            document.source().to_string(),
            chunk.metadata().category().to_string(),
            document.path().to_string_lossy().to_string(),
            document.tags().to_vec(),
            chunk.hash().to_string(),
            "tfidf-1.0".to_string(),
            document
                .metadata()
                .author()
                .unwrap_or("unknown")
                .to_string(),
            document.title().to_string(),
        )
    }

    fn chunks_from_search_results(&self, results: &SearchResultSet) -> Vec<Chunk> {
        let chunk_map = self.chunk_map.read();
        results
            .results
            .iter()
            .filter_map(|result| chunk_map.get(&result.vector_id).cloned())
            .collect()
    }

    fn documents_from_chunks(
        &self,
        storage: &dyn KnowledgeStorage,
        chunks: &[Chunk],
    ) -> Result<Vec<Document>, String> {
        let mut docs_by_id = HashMap::new();
        for document in storage
            .list_documents()
            .map_err(|e| format!("storage error: {}", e))?
        {
            docs_by_id.insert(document.id().to_string(), document);
        }

        let mut ordered = Vec::new();
        let mut seen = HashMap::new();
        for chunk in chunks {
            if let Some(doc) = docs_by_id.get(chunk.document_id()) {
                if !seen.contains_key(chunk.document_id()) {
                    ordered.push(doc.clone());
                    seen.insert(chunk.document_id().to_string(), true);
                }
            }
        }
        Ok(ordered)
    }

    fn record_metric_counter(&self, key: &str, value: u64) {
        if let Some(metrics) = &self.metrics {
            metrics.record_histogram(key, value, None);
        }
    }
}

impl KnowledgeRetrievalEngine for KnowledgeEmbeddingRetrievalEngine {
    fn retrieve(
        &self,
        storage: &dyn KnowledgeStorage,
        query: &str,
        limit: usize,
    ) -> KnowledgeResult<RetrievalResult> {
        let query_start = Instant::now();
        self.rebuild_index(storage)?;

        let semantic_results = self.semantic_engine.search(query, limit).map_err(|e| {
            crate::knowledge::errors::KnowledgeError::Runtime(format!("semantic search failed: {}", e))
        })?;

        let keyword_results = self.keyword_engine.search(query, limit).map_err(|e| {
            crate::knowledge::errors::KnowledgeError::Runtime(format!("keyword search failed: {}", e))
        })?;

        let rerank_start = Instant::now();
        let combined_search = self
            .hybrid_engine
            .combine_results(&semantic_results, &keyword_results, limit);

        // Apply deterministic multi-feature reranker
        let reranker = crate::knowledge::embedding::retrieval::reranker::SearchResultReranker::new(
            crate::knowledge::embedding::retrieval::reranker::RerankingStrategy::Multi,
        );
        let reranked = reranker.rerank(combined_search);

        let prompt_context = ContextBuilder::build_context(&semantic_results, 512);

        if let Some(metrics) = &self.metrics {
            metrics.incr("embedding_cache_miss", None);
            metrics.record_duration("retrieval_rerank_duration", rerank_start.elapsed(), None);
        }

        let retrieval_duration = query_start.elapsed();
        if let Some(metrics) = &self.metrics {
            metrics.record_duration("retrieval_latency", retrieval_duration, None);
            metrics.incr("top_k_size", Some(&[("k", &limit.to_string())]));
            metrics.incr("similarity_score", Some(&[("score", &format!("{:.2}", {
                let chunks = self.chunks_from_search_results(&reranked);
                chunks.first().map(|c| c.content().len()).unwrap_or(0)
            }))]));
        }

        let chunks = self.chunks_from_search_results(&reranked);
        let mut result = RetrievalResult::new(
            query.to_string(),
            self.documents_from_chunks(storage, &chunks)?,
            chunks,
            reranked.total_candidates,
        );
        // prompt_context available for runtime via metrics/logs; not stored on RetrievalResult
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::builder::KnowledgePipelineBuilder;
    use crate::knowledge::manager::KnowledgeManagerImpl;
    use crate::knowledge::storage::InMemoryKnowledgeStorage;
    use crate::knowledge::traits::KnowledgeManager;
    use crate::knowledge::metadata::{DocumentMetadata, DocumentMetadataSpec};
    use crate::knowledge::document::Document;
    use chrono::Utc;
    use std::path::PathBuf;

    fn build_document(id: &str, content: &str) -> Document {
        let now = Utc::now();
        Document::from_spec(crate::knowledge::document::DocumentSpec {
            id: id.to_string(),
            path: PathBuf::from("/tmp"),
            title: "test".to_string(),
            language: "en".to_string(),
            tags: vec!["test".to_string()],
            source: "local".to_string(),
            metadata: DocumentMetadata::from_spec(DocumentMetadataSpec {
                title: "test".to_string(),
                author: Some("author".to_string()),
                language: "en".to_string(),
                category: "test".to_string(),
                topic: None,
                tags: vec!["test".to_string()],
                difficulty: "easy".to_string(),
                version: "1.0".to_string(),
                source: "local".to_string(),
                hash: id.to_string(),
                size: content.len() as u64,
                created_at: now,
                updated_at: now,
            }),
            content: content.to_string(),
        })
    }

    #[test]
    fn embedding_retrieval_engine_returns_top_chunks() {
        let storage = InMemoryKnowledgeStorage::new();
        let document = build_document("d1", "rust ownership and borrowing");
        storage.save_document(document.clone()).unwrap();
        let chunk = crate::knowledge::chunk::Chunk::new(
            "c1".to_string(),
            "d1".to_string(),
            0,
            "rust ownership rules".to_string(),
            document.metadata().clone(),
            "h1".to_string(),
        );
        storage.save_chunks(vec![chunk]).unwrap();

        let engine = KnowledgeEmbeddingRetrievalEngine::new(EmbeddingRetrievalConfig::default())
            .expect("build embedding retrieval engine");
        let result = engine
            .retrieve(&storage, "ownership", 5)
            .expect("retrieve query");

        assert!(result.document_count >= 1);
        assert!(!result.chunks.is_empty());
    }
}
