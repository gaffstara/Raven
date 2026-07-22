use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::traits::DocumentValidator;
use std::fs;
use std::path::Path;

/// Validator that checks document source file integrity and supported format.
#[derive(Debug)]
pub struct FileValidator;

impl FileValidator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentValidator for FileValidator {
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

        let metadata = fs::metadata(path)?;
        if metadata.len() == 0 {
            return Err(KnowledgeError::ValidationFailed(format!(
                "file is empty: {}",
                path.display()
            )));
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_lowercase();
        if !file_name.ends_with(".md") && !file_name.ends_with(".txt") {
            return Err(KnowledgeError::UnsupportedFormat(file_name));
        }

        // Allow documents without frontmatter. Detailed metadata
        // validation is performed later by the pipeline when processing
        // official knowledge library files. For loader/tests and simple
        // files, just ensure the file has readable content.
        let _content = fs::read_to_string(path)?;

        Ok(())
    }
}
