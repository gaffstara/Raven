use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use std::collections::HashSet;

/// Simple keyword-based taxonomy classifier. Extensible to ML later.
pub struct TaxonomyClassifier;

impl TaxonomyClassifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn classify(&self, text: &str) -> KnowledgeResult<Vec<String>> {
        let mut tags = HashSet::new();
        let lower = text.to_lowercase();
        if lower.contains("ownership") || lower.contains("borrow") || lower.contains("lifetime") {
            tags.insert("programming/rust/ownership".to_string());
        }
        if lower.contains("concurrent") || lower.contains("goroutine") || lower.contains("thread") {
            tags.insert("programming/concurrency".to_string());
        }
        if lower.contains("async") || lower.contains("await") {
            tags.insert("programming/async".to_string());
        }
        if lower.contains("test") || lower.contains("unittest") {
            tags.insert("programming/testing".to_string());
        }
        if lower.contains("cargo") || lower.contains("crate") {
            tags.insert("programming/rust/cargo".to_string());
        }

        Ok(tags.into_iter().collect())
    }
}
