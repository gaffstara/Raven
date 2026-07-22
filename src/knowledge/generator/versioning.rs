use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::traits::KnowledgeStorage;

/// Versioning: if hash matches existing document, return existing id, else new.
pub struct Versioning<'a> {
    storage: &'a dyn KnowledgeStorage,
}

impl<'a> Versioning<'a> {
    pub fn new(storage: &'a dyn KnowledgeStorage) -> Self {
        Self { storage }
    }

    pub fn detect_version(&self, hash: &str) -> KnowledgeResult<Option<String>> {
        let docs = self.storage.list_documents()?;
        for d in docs {
            if d.metadata().hash() == hash {
                return Ok(Some(d.id().to_string()));
            }
        }
        Ok(None)
    }
}
