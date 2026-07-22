use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::traits::HashEngine;
use crate::knowledge::traits::KnowledgeStorage;
use crate::knowledge::chunk::Chunk;

/// Deduplicator checks chunk hashes against storage to avoid duplicates.
pub struct Deduplicator<'a> {
    storage: &'a dyn KnowledgeStorage,
    hasher: &'a dyn HashEngine,
}

impl<'a> Deduplicator<'a> {
    pub fn new(storage: &'a dyn KnowledgeStorage, hasher: &'a dyn HashEngine) -> Self {
        Self { storage, hasher }
    }

    pub fn filter_duplicates(&self, chunks: Vec<Chunk>) -> KnowledgeResult<Vec<Chunk>> {
        let mut out = Vec::new();
        for chunk in chunks {
            let h = self.hasher.hash(chunk.content().as_bytes());
            // simple duplicate detection: compare to existing chunk hashes
            let existing = self.storage.list_documents()?;
            let mut is_dup = false;
            for doc in existing {
                let existing_chunks = self.storage.list_chunks(doc.id())?;
                for ec in existing_chunks {
                    let eh = self.hasher.hash(ec.content().as_bytes());
                    if eh == h {
                        is_dup = true;
                        break;
                    }
                }
                if is_dup {
                    break;
                }
            }
            if !is_dup {
                out.push(chunk);
            }
        }
        Ok(out)
    }
}
