use crate::knowledge::generator::cleaner::Cleaner;
use crate::knowledge::generator::parser::SimpleParser;
use crate::knowledge::generator::language::LanguageDetector;
use crate::knowledge::generator::metadata::MetadataExtractor;
use crate::knowledge::generator::chunker::SemanticChunker;
use crate::knowledge::generator::deduplicator::Deduplicator;
use crate::knowledge::generator::hasher::GeneratorHasher;
use crate::knowledge::generator::taxonomy::TaxonomyClassifier;
use crate::knowledge::generator::linker::Linker;
use crate::knowledge::generator::versioning::Versioning;
use crate::knowledge::generator::report::GeneratorReport;
use crate::knowledge::pipeline::KnowledgePipeline;
use crate::knowledge::traits::{KnowledgeStorage, HashEngine};
use crate::knowledge::errors::KnowledgeResult;
use std::path::Path;
use std::time::Instant;

/// KnowledgeGenerator orchestrates conversion of raw files into Knowledge Library
pub struct KnowledgeGenerator {
    pipeline: KnowledgePipeline,
}

impl KnowledgeGenerator {
    pub fn new(pipeline: KnowledgePipeline) -> Self {
        Self { pipeline }
    }

    pub fn process_file(&self, path: &Path) -> KnowledgeResult<String> {
        let start = Instant::now();

        // Step: validate (generator uses pipeline validator)
        self.pipeline.validator.validate(path)?;

        // Step: load
        let mut document = self.pipeline.loader.load(path)?;

        // Step: clean
        let cleaner = Cleaner::new();
        let cleaned = cleaner.clean(document.content())?;

        // Step: detect language
        let lang = LanguageDetector::new().detect(path, &cleaned);

        // Step: metadata extraction
        let meta_map = MetadataExtractor::new().extract(path, &cleaned)?;

        // Step: parser extract title and sections
        let parser = SimpleParser::new();
        let title = parser.extract_title(&cleaned).unwrap_or_else(|| document.title().to_string());
        // update document with cleaned content and language
        let mut document = crate::knowledge::document::Document::from_spec(
            crate::knowledge::document::DocumentSpec {
                id: document.id().to_string(),
                path: document.path().to_path_buf(),
                title: title.clone(),
                language: lang.clone(),
                tags: document.tags().to_vec(),
                source: document.source().to_string(),
                metadata: document.metadata().clone(),
                content: cleaned.clone(),
            },
        );

        // Step: chunk (semantic)
        let sem_chunker = SemanticChunker::new(1500);
        let mut chunks = sem_chunker.chunk_document(&document)?;

        // Step: hash document & chunks
        let hasher = GeneratorHasher::new();
        let doc_hash = hasher.hash(document.content().as_bytes());

        // update metadata hash
        // (rebuild metadata with new hash)
        let meta = document.metadata().clone();
        let new_meta = crate::knowledge::metadata::DocumentMetadata::from_spec(
            crate::knowledge::metadata::DocumentMetadataSpec {
                title: meta.title().to_string(),
                author: meta.author().map(|s| s.to_string()),
                language: lang,
                category: meta.category().to_string(),
                topic: meta.topic().map(|s| s.to_string()),
                tags: meta.tags().to_vec(),
                difficulty: meta.difficulty().to_string(),
                version: meta.version().to_string(),
                source: meta.source().to_string(),
                hash: doc_hash.clone(),
                size: document.content().len() as u64,
                created_at: meta.created_at(),
                updated_at: meta.updated_at(),
            },
        );
        document = crate::knowledge::document::Document::from_spec(
            crate::knowledge::document::DocumentSpec {
                id: document.id().to_string(),
                path: document.path().to_path_buf(),
                title: document.title().to_string(),
                language: new_meta.language().to_string(),
                tags: new_meta.tags().to_vec(),
                source: new_meta.source().to_string(),
                metadata: new_meta,
                content: document.content().to_string(),
            },
        );

        // Step: deduplicate chunks
        let dedup = Deduplicator::new(self.pipeline.storage.as_ref(), self.pipeline.hash_engine.as_ref());
        let filtered = dedup.filter_duplicates(chunks)?;

        // Step: taxonomy
        let classifier = TaxonomyClassifier::new();
        let tags = classifier.classify(document.content())?;

        // Step: link
        let linker = Linker::new(self.pipeline.storage.as_ref());
        // store document first
        self.pipeline.storage.save_document(document.clone())?;
        // save chunks
        self.pipeline.storage.save_chunks(filtered.clone())?;

        let related = linker.find_related(document.id())?;

        // Step: versioning
        let versioning = Versioning::new(self.pipeline.storage.as_ref());
        let _existing = versioning.detect_version(document.metadata().hash())?;

        // Step: report and metrics
        let mut report = GeneratorReport::default();
        report.files_processed = 1;
        report.chunks_generated = filtered.len();
        report.knowledge_stored = 1;
        report.duration = Some(start.elapsed());

        // Return document id as success
        Ok(document.id().to_string())
    }

    pub fn process_directory(&self, root: &Path) -> KnowledgeResult<Vec<String>> {
        let mut processed = Vec::new();
        for entry in root.read_dir().map_err(|e| crate::knowledge::errors::KnowledgeError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| crate::knowledge::errors::KnowledgeError::Io(e.to_string()))?;
            let path = entry.path();
            if path.is_dir() {
                let mut child = self.process_directory(&path)?;
                processed.append(&mut child);
            } else if path.is_file() {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("txt") || ext.eq_ignore_ascii_case("rs") || ext.eq_ignore_ascii_case("py") || ext.eq_ignore_ascii_case("go") || ext.eq_ignore_ascii_case("java") || ext.eq_ignore_ascii_case("js") {
                    if let Ok(id) = self.process_file(&path) {
                        processed.push(id);
                    }
                }
            }
        }
        Ok(processed)
    }
}
