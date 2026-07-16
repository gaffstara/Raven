//! Memory entry validator based on rules.

use crate::ecc::memory::errors::MemoryEccResult;
use crate::ecc::memory::rules::*;
use crate::ecc::memory::types::MemoryValidationEntry;
use crate::ecc::report::ValidationReport;
use crate::ecc::traits::{Rule, Validator};
use chrono::Utc;
use std::time::Instant;

/// Rule-based validator for memory entries.
///
/// This validator combines multiple validation rules to comprehensively
/// check memory entries before they are stored, updated, or retrieved.
pub struct MemoryValidator {
    rules: Vec<Box<dyn Rule<MemoryValidationEntry>>>,
}

impl MemoryValidator {
    /// Create a new memory validator with all 18 standard rules.
    pub fn new() -> Self {
        let rules: Vec<Box<dyn Rule<MemoryValidationEntry>>> = vec![
            // ID rules
            Box::new(MemoryIdFormatRule),
            Box::new(MemoryIdNotEmptyRule),
            // Timestamp rules
            Box::new(TimestampValidRule),
            // Kind rules
            Box::new(MemoryKindValidRule),
            // Text rules
            Box::new(TextNotEmptyRule),
            Box::new(TextUtf8ValidRule),
            Box::new(TextMaxLengthRule),
            // Importance rules
            Box::new(ImportanceInRangeRule),
            Box::new(ImportanceNotNaNRule),
            // Tag rules
            Box::new(TagsNotEmptyRule),
            Box::new(TagsNoDuplicatesRule),
            Box::new(TagsValidStringsRule),
            // Metadata rules
            Box::new(MetadataJsonValidRule),
            Box::new(MetadataKeysValidRule),
            Box::new(EmbeddingMetadataValidRule),
            // Source and checksum rules
            Box::new(SourceValidRule),
            Box::new(ChecksumFormatValidRule),
            // Structural rules
            Box::new(NoDuplicateReferencesRule),
            Box::new(RequiredFieldsPresentRule),
        ];

        Self { rules }
    }

    /// Create a custom validator with a specific set of rules.
    pub fn with_rules(rules: Vec<Box<dyn Rule<MemoryValidationEntry>>>) -> Self {
        Self { rules }
    }

    /// Get the IDs of all rules in this validator.
    pub fn rule_ids(&self) -> Vec<&'static str> {
        self.rules.iter().map(|r| r.id()).collect()
    }
}

impl Default for MemoryValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator<MemoryValidationEntry> for MemoryValidator {
    fn validate(&self, subject: &MemoryValidationEntry) -> MemoryEccResult<ValidationReport> {
        let start = Instant::now();
        let mut issues = Vec::new();

        for rule in &self.rules {
            if rule.applies_to(subject) {
                let mut rule_issues = rule.evaluate(subject)?;
                issues.append(&mut rule_issues);
            }
        }

        let duration = start.elapsed();
        let report = ValidationReport::new(Utc::now(), duration, issues);

        Ok(report)
    }

    fn rule_ids(&self) -> Vec<&'static str> {
        self.rule_ids()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{MemoryEntry, MemoryKind};
    use chrono::Utc;

    fn create_valid_entry() -> MemoryValidationEntry {
        let entry = MemoryEntry {
            id: "m00000001".to_string(),
            kind: MemoryKind::Working,
            text: "This is valid text".to_string(),
            created_at: Utc::now(),
            tags: vec!["test".to_string()],
            importance: 0.5,
        };

        MemoryValidationEntry::from_entry(entry)
    }

    #[test]
    fn test_validator_with_valid_entry() {
        let validator = MemoryValidator::new();
        let entry = create_valid_entry();

        let result = validator.validate(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(report.is_valid);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn test_validator_with_empty_text() {
        let validator = MemoryValidator::new();
        let mut entry = create_valid_entry();
        entry.entry.text = "   ".to_string();

        let result = validator.validate(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(!report.is_valid);
        assert!(!report.issues.is_empty());
    }

    #[test]
    fn test_validator_with_invalid_id_format() {
        let validator = MemoryValidator::new();
        let mut entry = create_valid_entry();
        entry.entry.id = "invalid".to_string();

        let result = validator.validate(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(!report.is_valid);
    }

    #[test]
    fn test_validator_with_out_of_range_importance() {
        let validator = MemoryValidator::new();
        let mut entry = create_valid_entry();
        entry.entry.importance = 1.5;

        let result = validator.validate(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(!report.is_valid);
    }

    #[test]
    fn test_validator_with_empty_tags() {
        let validator = MemoryValidator::new();
        let mut entry = create_valid_entry();
        entry.entry.tags = Vec::new();

        let result = validator.validate(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(!report.is_valid);
    }

    #[test]
    fn test_validator_with_duplicate_tags() {
        let validator = MemoryValidator::new();
        let mut entry = create_valid_entry();
        entry.entry.tags = vec!["test".to_string(), "test".to_string()];

        let result = validator.validate(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(!report.is_valid);
    }
}
