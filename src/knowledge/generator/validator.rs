use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::traits::DocumentValidator;
use std::path::Path;

/// Validator tailored for generator needs: ensure file exists and reasonable size.
#[derive(Debug)]
pub struct GeneratorValidator;

impl GeneratorValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl DocumentValidator for GeneratorValidator {
    fn validate(&self, path: &Path) -> KnowledgeResult<()> {
        if !path.exists() {
            return Err(KnowledgeError::ValidationFailed(format!(
                "file does not exist: {}",
                path.display()
            )));
        }
        if !path.is_file() {
            return Err(KnowledgeError::ValidationFailed(format!(
                "path is not a file: {}",
                path.display()
            )));
        }
        let metadata = std::fs::metadata(path)?;
        if metadata.len() == 0 {
            return Err(KnowledgeError::ValidationFailed(format!(
                "file is empty: {}",
                path.display()
            )));
        }
        Ok(())
    }
}
