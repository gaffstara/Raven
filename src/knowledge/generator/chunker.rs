use crate::knowledge::chunk::Chunk;
use crate::knowledge::document::Document;
use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::metadata::{DocumentMetadata, DocumentMetadataSpec};

/// Semantic chunker that prioritizes headings and code blocks.
pub struct SemanticChunker {
    max_chars: usize,
}

impl SemanticChunker {
    pub fn new(max_chars: usize) -> Self {
        Self { max_chars }
    }

    pub fn chunk_document(&self, document: &Document) -> KnowledgeResult<Vec<Chunk>> {
        let content = document.content();
        // split by headings first
        let mut chunks = Vec::new();
        let mut seq = 0usize;

        let mut current = String::new();
        for line in content.lines() {
            if line.trim().starts_with('#') {
                if !current.is_empty() {
                    let chunk = Self::make_chunk(document, seq, &current)?;
                    chunks.push(chunk);
                    seq += 1;
                    current.clear();
                }
                current.push_str(line);
                current.push('\n');
            } else {
                current.push_str(line);
                current.push('\n');
                if current.len() >= self.max_chars {
                    let chunk = Self::make_chunk(document, seq, &current)?;
                    chunks.push(chunk);
                    seq += 1;
                    current.clear();
                }
            }
        }

        if !current.is_empty() {
            let chunk = Self::make_chunk(document, seq, &current)?;
            chunks.push(chunk);
        }

        if chunks.is_empty() {
            return Err(KnowledgeError::ValidationFailed(format!(
                "unable to chunk document {}: no chunks produced",
                document.id()
            )));
        }
        Ok(chunks)
    }

    fn make_chunk(document: &Document, seq: usize, content: &str) -> KnowledgeResult<Chunk> {
        let meta = DocumentMetadata::from_spec(DocumentMetadataSpec {
            title: document.title().to_string(),
            author: document.metadata().author().map(|s| s.to_string()),
            language: document.language().to_string(),
            category: document.metadata().category().to_string(),
            topic: document.metadata().topic().map(|s| s.to_string()),
            tags: document.metadata().tags().to_vec(),
            difficulty: document.metadata().difficulty().to_string(),
            version: document.metadata().version().to_string(),
            source: document.metadata().source().to_string(),
            hash: document.metadata().hash().to_string(),
            size: content.len() as u64,
            created_at: document.metadata().created_at(),
            updated_at: document.metadata().updated_at(),
        });

        Ok(Chunk::new(
            format!("{}:chunk:{}", document.id(), seq),
            document.id().to_string(),
            seq,
            content.to_string(),
            meta,
            String::new(),
        ))
    }
}
