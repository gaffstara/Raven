use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::traits::KnowledgeStorage;

/// Build simple cross-references between documents based on shared tags or headings.
pub struct Linker<'a> {
    storage: &'a dyn KnowledgeStorage,
}

impl<'a> Linker<'a> {
    pub fn new(storage: &'a dyn KnowledgeStorage) -> Self {
        Self { storage }
    }

    pub fn find_related(&self, doc_id: &str) -> KnowledgeResult<Vec<String>> {
        let mut related = Vec::new();
        let docs = self.storage.list_documents()?;
        let target = self.storage.get_document(doc_id)?;
        if target.is_none() {
            return Ok(related);
        }
        let target = target.unwrap();
        for doc in docs {
            if doc.id() == doc_id {
                continue;
            }
            // simple matching: any shared tag
            for tag in target.tags() {
                if doc.tags().contains(tag) {
                    related.push(doc.id().to_string());
                    break;
                }
            }
        }
        Ok(related)
    }
}
