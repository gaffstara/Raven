//! Types for Memory ECC subsystem.

use crate::memory::MemoryEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Extended memory entry with additional validation metadata.
/// This wraps the core MemoryEntry with ECC-specific fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryValidationEntry {
    /// The base memory entry from the memory system
    pub entry: MemoryEntry,

    /// Optional metadata for checksum validation
    pub checksum: Option<String>,

    /// Optional source identifier for audit trail
    pub source: Option<String>,

    /// Optional embedding metadata for semantic memory
    pub embedding_metadata: Option<EmbeddingMetadata>,

    /// Additional metadata as JSON map
    pub metadata: serde_json::Map<String, serde_json::Value>,

    /// Timestamp when entry was validated
    pub validated_at: Option<DateTime<Utc>>,

    /// Whether this entry has been validated before
    pub ever_validated: bool,
}

impl MemoryValidationEntry {
    /// Create a new MemoryValidationEntry from a MemoryEntry.
    pub fn from_entry(entry: MemoryEntry) -> Self {
        Self {
            entry,
            checksum: None,
            source: None,
            embedding_metadata: None,
            metadata: serde_json::Map::new(),
            validated_at: None,
            ever_validated: false,
        }
    }

    /// Create with all fields.
    #[allow(clippy::too_many_arguments)]
    pub fn with_metadata(
        entry: MemoryEntry,
        checksum: Option<String>,
        source: Option<String>,
        embedding_metadata: Option<EmbeddingMetadata>,
        metadata: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            entry,
            checksum,
            source,
            embedding_metadata,
            metadata,
            validated_at: None,
            ever_validated: false,
        }
    }

    /// Mark this entry as validated at the given timestamp.
    pub fn mark_validated(&mut self, at: DateTime<Utc>) {
        self.validated_at = Some(at);
        self.ever_validated = true;
    }
}

/// Metadata for semantic embeddings in memory entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingMetadata {
    /// Vector dimension (e.g., 768 for BERT)
    pub dimension: usize,

    /// Model identifier (e.g., "bert-base-uncased")
    pub model: String,

    /// Optional hash of the embedding vector
    pub hash: Option<String>,

    /// Timestamp when embedding was generated
    pub generated_at: DateTime<Utc>,
}

impl EmbeddingMetadata {
    /// Create new embedding metadata.
    pub fn new(dimension: usize, model: String) -> Self {
        Self {
            dimension,
            model,
            hash: None,
            generated_at: Utc::now(),
        }
    }

    /// Create with hash.
    pub fn with_hash(dimension: usize, model: String, hash: String) -> Self {
        Self {
            dimension,
            model,
            hash: Some(hash),
            generated_at: Utc::now(),
        }
    }
}

/// Enumeration of memory validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryIssueKind {
    // ID issues
    MemoryIdFormat,
    MemoryIdEmpty,

    // Timestamp issues
    TimestampInvalid,
    TimestampInFuture,

    // Kind issues
    MemoryKindInvalid,

    // Text issues
    TextEmpty,
    TextNotUtf8,
    TextTooLong,

    // Importance issues
    ImportanceOutOfRange,
    ImportanceNaN,

    // Tag issues
    TagsEmpty,
    TagsDuplicate,
    TagsInvalid,

    // Metadata issues
    MetadataInvalid,
    MetadataKeysInvalid,
    EmbeddingMetadataInvalid,

    // Reference issues
    SourceInvalid,
    ChecksumFormatInvalid,
    DuplicateReferences,

    // Structural issues
    RequiredFieldsMissing,
}

impl MemoryIssueKind {
    /// Get the category for this issue kind.
    pub fn category(&self) -> &'static str {
        match self {
            Self::MemoryIdFormat | Self::MemoryIdEmpty => "structure.id",
            Self::TimestampInvalid | Self::TimestampInFuture => "structure.timestamp",
            Self::MemoryKindInvalid => "structure.kind",
            Self::TextEmpty | Self::TextNotUtf8 | Self::TextTooLong => "structure.text",
            Self::ImportanceOutOfRange | Self::ImportanceNaN => "structure.importance",
            Self::TagsEmpty | Self::TagsDuplicate | Self::TagsInvalid => "structure.tags",
            Self::MetadataInvalid | Self::MetadataKeysInvalid => "structure.metadata",
            Self::EmbeddingMetadataInvalid => "structure.embedding",
            Self::SourceInvalid => "structure.source",
            Self::ChecksumFormatInvalid => "structure.checksum",
            Self::DuplicateReferences => "structure.references",
            Self::RequiredFieldsMissing => "structure.required",
        }
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::MemoryIdFormat => "Memory ID format is invalid",
            Self::MemoryIdEmpty => "Memory ID is empty",
            Self::TimestampInvalid => "Timestamp is invalid or malformed",
            Self::TimestampInFuture => "Timestamp is in the future",
            Self::MemoryKindInvalid => "Memory kind is not a valid MemoryKind variant",
            Self::TextEmpty => "Memory text content is empty",
            Self::TextNotUtf8 => "Memory text is not valid UTF-8",
            Self::TextTooLong => "Memory text exceeds maximum length",
            Self::ImportanceOutOfRange => "Importance score is not in range [0.0, 1.0]",
            Self::ImportanceNaN => "Importance score is NaN",
            Self::TagsEmpty => "Memory has no tags",
            Self::TagsDuplicate => "Memory has duplicate tags",
            Self::TagsInvalid => "Memory has invalid tags (empty strings, etc.)",
            Self::MetadataInvalid => "JSON metadata is invalid or malformed",
            Self::MetadataKeysInvalid => "Metadata contains empty keys",
            Self::EmbeddingMetadataInvalid => "Embedding metadata structure is invalid",
            Self::SourceInvalid => "Source field is invalid or empty",
            Self::ChecksumFormatInvalid => "Checksum format is invalid",
            Self::DuplicateReferences => "Duplicate references detected in memory",
            Self::RequiredFieldsMissing => "Required fields are missing from entry",
        }
    }
}

/// Classification levels for memory validation results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryValidationLevel {
    /// All validation rules passed, no issues detected
    Valid,

    /// Minor formatting issues that can be auto-corrected
    MinorIssue,

    /// Issues that can be fixed with high confidence
    Recoverable,

    /// Issues that cannot be automatically fixed
    NonRecoverable,

    /// Severe corruption, entry is unusable
    Corrupted,
}

impl MemoryValidationLevel {
    /// Get severity score (0.0 = no risk, 1.0 = severe).
    pub fn severity_score(&self) -> f32 {
        match self {
            Self::Valid => 0.0,
            Self::MinorIssue => 0.1,
            Self::Recoverable => 0.4,
            Self::NonRecoverable => 0.7,
            Self::Corrupted => 1.0,
        }
    }
}

/// Maximum length for memory text content (in bytes).
pub const MEMORY_TEXT_MAX_BYTES: usize = 1_000_000; // 1 MB

/// Minimum reasonable memory entry size (in bytes).
pub const MEMORY_TEXT_MIN_BYTES: usize = 1;

/// Maximum number of tags per memory entry.
pub const MEMORY_MAX_TAGS: usize = 50;

/// Maximum length for a single tag (in bytes).
pub const MEMORY_TAG_MAX_BYTES: usize = 100;
