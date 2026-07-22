use crate::knowledge::chunk::Chunk;
use crate::knowledge::document::Document;
use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::metadata::{DocumentMetadata, DocumentMetadataSpec};
use crate::knowledge::traits::Chunker;

/// Deterministic chunker splitting content into fixed-size sequential chunks.
#[derive(Debug)]
pub struct FixedChunker {
    max_chunk_size: usize,
}

impl FixedChunker {
    pub fn new(max_chunk_size: usize) -> Self {
        Self { max_chunk_size }
    }
}

impl Chunker for FixedChunker {
    fn chunk(&self, document: &Document) -> KnowledgeResult<Vec<Chunk>> {
        let content = document.content();
        let mut chunks = Vec::new();
        let mut offset = 0;
        let bytes = content.as_bytes();
        let mut sequence = 0;

        while offset < bytes.len() {
            let end = usize::min(offset + self.max_chunk_size, bytes.len());
            let slice = &bytes[offset..end];
            let chunk_content = String::from_utf8(slice.to_vec()).map_err(|err| {
                KnowledgeError::ValidationFailed(format!(
                    "chunk content encoding invalid for document {}: {}",
                    document.id(),
                    err
                ))
            })?;

            let chunk_metadata = DocumentMetadata::from_spec(DocumentMetadataSpec {
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
                size: chunk_content.len() as u64,
                created_at: document.metadata().created_at(),
                updated_at: document.metadata().updated_at(),
            });

            let chunk = Chunk::new(
                format!("{}:chunk:{}", document.id(), sequence),
                document.id().to_string(),
                sequence,
                chunk_content,
                chunk_metadata,
                String::new(),
            );

            chunks.push(chunk);
            sequence += 1;
            offset = end;
        }

        if chunks.is_empty() {
            return Err(KnowledgeError::ValidationFailed(format!(
                "unable to chunk document {}: no content",
                document.id()
            )));
        }

        Ok(chunks)
    }
}
