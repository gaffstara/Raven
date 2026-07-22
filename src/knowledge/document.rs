use crate::knowledge::metadata::DocumentMetadata;
use std::path::{Path, PathBuf};

/// Document model representing a knowledge artifact.
#[derive(Clone, Debug)]
pub struct Document {
    id: String,
    path: PathBuf,
    title: String,
    language: String,
    tags: Vec<String>,
    source: String,
    metadata: DocumentMetadata,
    content: String,
}

pub struct DocumentSpec {
    pub id: String,
    pub path: PathBuf,
    pub title: String,
    pub language: String,
    pub tags: Vec<String>,
    pub source: String,
    pub metadata: DocumentMetadata,
    pub content: String,
}

impl Document {
    pub fn from_spec(spec: DocumentSpec) -> Self {
        Self {
            id: spec.id,
            path: spec.path,
            title: spec.title,
            language: spec.language,
            tags: spec.tags,
            source: spec.source,
            metadata: spec.metadata,
            content: spec.content,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn metadata(&self) -> &DocumentMetadata {
        &self.metadata
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
