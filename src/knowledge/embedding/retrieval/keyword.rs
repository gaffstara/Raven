//! Keyword search engine using Tantivy for full-text chunk retrieval.

use crate::knowledge::embedding::vector::search::SearchResultSet;
use crate::knowledge::traits::KnowledgeStorage;
use std::sync::{Arc, Mutex};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexReader, IndexWriter};

/// Keyword search engine backed by Tantivy.
pub struct KeywordSearchEngine {
    index: Index,
    reader: IndexReader,
    chunk_id_field: Field,
    document_id_field: Field,
    content_field: Field,
    title_field: Field,
    tags_field: Field,
    writer_lock: Arc<Mutex<()>>,
}

impl KeywordSearchEngine {
    /// Create a new keyword search engine with a simple schema.
    pub fn new() -> Result<Self, String> {
        let mut schema_builder = Schema::builder();
        let chunk_id_field = schema_builder.add_text_field("chunk_id", STRING | STORED);
        let document_id_field = schema_builder.add_text_field("document_id", STRING | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let tags_field = schema_builder.add_text_field("tags", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());
        let reader = index
            .reader()
            .map_err(|e| format!("Tantivy reader creation failed: {}", e))?;

        Ok(Self {
            index,
            reader,
            chunk_id_field,
            document_id_field,
            content_field,
            title_field,
            tags_field,
            writer_lock: Arc::new(Mutex::new(())),
        })
    }

    fn writer(&self) -> Result<IndexWriter, String> {
        self.index
            .writer(50_000_000)
            .map_err(|e| format!("Tantivy writer creation failed: {}", e))
    }

    /// Rebuild the keyword index from the current storage contents.
    pub fn rebuild(&self, storage: &dyn KnowledgeStorage) -> Result<(), String> {
        let _guard = self.writer_lock.lock().map_err(|e| e.to_string())?;
        let mut writer = self.writer()?;
        writer
            .delete_all_documents()
            .map_err(|e| format!("Tantivy delete failed: {}", e))?;

        let documents = storage
            .list_documents()
            .map_err(|e| format!("storage error: {}", e))?;

        for document in documents {
            let chunks = storage
                .list_chunks(document.id())
                .map_err(|e| format!("storage error: {}", e))?;
            for chunk in chunks {
                writer
                    .add_document(doc!(
                        self.chunk_id_field => chunk.id().to_string(),
                        self.document_id_field => chunk.document_id().to_string(),
                        self.title_field => chunk.metadata().title().to_string(),
                        self.tags_field => chunk.metadata().tags().join(" "),
                        self.content_field => chunk.content().to_string(),
                    ))
                    .map_err(|e| format!("Tantivy write failed: {}", e))?;
            }
        }

        writer
            .commit()
            .map_err(|e| format!("Tantivy commit failed: {}", e))?;
        self.reader
            .reload()
            .map_err(|e| format!("Tantivy reader reload failed: {}", e))?;
        Ok(())
    }

    /// Search the keyword index and return ranked results.
    pub fn search(&self, query: &str, limit: usize) -> Result<SearchResultSet, String> {
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.content_field, self.title_field, self.tags_field],
        );
        let parsed_query = query_parser
            .parse_query(query)
            .map_err(|e| format!("Tantivy parse query failed: {}", e))?;

        let top_docs = searcher
            .search(&parsed_query, &TopDocs::with_limit(limit))
            .map_err(|e| format!("Tantivy search failed: {}", e))?;

        let mut results = Vec::new();
        let mut max_score = 0.0_f32;

        for (score, _) in &top_docs {
            max_score = max_score.max(*score);
        }

        for (score, doc_address) in top_docs {
            let retrieved = searcher
                .doc(doc_address)
                .map_err(|e| format!("Tantivy retrieve doc failed: {}", e))?;

            let chunk_id = retrieved
                .get_first(self.chunk_id_field)
                .and_then(|field_value| field_value.as_text())
                .unwrap_or_default()
                .to_string();
            let document_id = retrieved
                .get_first(self.document_id_field)
                .and_then(|field_value| field_value.as_text())
                .unwrap_or_default()
                .to_string();
            let content = retrieved
                .get_first(self.content_field)
                .and_then(|field_value| field_value.as_text())
                .unwrap_or_default()
                .to_string();
            let title = retrieved
                .get_first(self.title_field)
                .and_then(|field_value| field_value.as_text())
                .unwrap_or_default()
                .to_string();
            let tags = retrieved
                .get_first(self.tags_field)
                .and_then(|field_value| field_value.as_text())
                .unwrap_or_default()
                .to_string();

            let mut metadata = std::collections::HashMap::new();
            metadata.insert("document_id".to_string(), document_id);
            metadata.insert("chunk_id".to_string(), chunk_id.clone());
            metadata.insert("title".to_string(), title);
            metadata.insert("tags".to_string(), tags);

            let normalized_score = if max_score > 0.0 {
                (score / max_score).clamp(0.0, 1.0)
            } else {
                0.0
            };

            results.push(crate::knowledge::embedding::vector::search::SearchResult {
                vector_id: chunk_id,
                similarity_score: normalized_score,
                content,
                metadata,
            });
        }

        let query_string = query.to_string();
        let total_candidates = results.len();
        Ok(SearchResultSet::new(
            results,
            query_string,
            limit,
            total_candidates,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::document::Document;
    use crate::knowledge::metadata::{DocumentMetadata, DocumentMetadataSpec};
    use crate::knowledge::storage::InMemoryKnowledgeStorage;
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
                author: None,
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
    fn keyword_search_engine_rebuilds_and_searches() {
        let storage = InMemoryKnowledgeStorage::new();
        let document = build_document("d1", "rust code and ownership");
        storage.save_document(document.clone()).unwrap();
        let chunk = crate::knowledge::chunk::Chunk::new(
            "c1".to_string(),
            "d1".to_string(),
            0,
            "rust ownership rules".to_string(),
            document.metadata().clone(),
            "hash1".to_string(),
        );
        storage.save_chunks(vec![chunk]).unwrap();

        let engine = KeywordSearchEngine::new().expect("keyword engine init");
        engine.rebuild(&storage).expect("rebuild keyword index");
        let results = engine.search("ownership", 5).expect("keyword search");
        assert!(!results.results.is_empty());
    }
}
