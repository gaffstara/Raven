use chrono::{DateTime, Utc};
use std::fmt;

/// Immutable metadata for a processed document.
#[derive(Clone, Debug)]
pub struct DocumentMetadata {
    title: String,
    author: Option<String>,
    language: String,
    category: String,
    topic: Option<String>,
    tags: Vec<String>,
    difficulty: String,
    version: String,
    source: String,
    hash: String,
    size: u64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct DocumentMetadataSpec {
    pub title: String,
    pub author: Option<String>,
    pub language: String,
    pub category: String,
    pub topic: Option<String>,
    pub tags: Vec<String>,
    pub difficulty: String,
    pub version: String,
    pub source: String,
    pub hash: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DocumentMetadata {
    pub fn from_spec(spec: DocumentMetadataSpec) -> Self {
        Self {
            title: spec.title,
            author: spec.author,
            language: spec.language,
            category: spec.category,
            topic: spec.topic,
            tags: spec.tags,
            difficulty: spec.difficulty,
            version: spec.version,
            source: spec.source,
            hash: spec.hash,
            size: spec.size,
            created_at: spec.created_at,
            updated_at: spec.updated_at,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn topic(&self) -> Option<&str> {
        self.topic.as_deref()
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn difficulty(&self) -> &str {
        &self.difficulty
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl fmt::Display for DocumentMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DocumentMetadata(title={}, language={}, tags={:?}, hash={}, size={})",
            self.title, self.language, self.tags, self.hash, self.size
        )
    }
}
