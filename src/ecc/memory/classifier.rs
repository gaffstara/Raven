//! Memory entry error classifier.

use crate::ecc::memory::errors::MemoryEccResult;
use crate::ecc::memory::types::MemoryValidationEntry;
use crate::ecc::pipeline::PipelineContext;
use crate::ecc::report::{EccIssue, ErrorClassification, ErrorSeverity};
use crate::ecc::traits::ErrorClassifier;

/// Classifier for memory validation issues.
pub struct MemoryErrorClassifier;

impl MemoryErrorClassifier {
    /// Create a new memory error classifier.
    pub fn new() -> Self {
        Self
    }

    /// Classify an issue based on its code and context.
    fn classify_issue(&self, issue: &EccIssue) -> ErrorClassification {
        let (category, severity, confidence) = self.classify_code(&issue.code);

        ErrorClassification {
            issue_code: issue.code.clone(),
            category,
            severity,
            confidence,
        }
    }

    /// Determine category and severity from issue code.
    fn classify_code(&self, code: &str) -> (String, ErrorSeverity, f32) {
        match code {
            // Structure issues - varying severity
            c if c.contains("memory_id") || c.contains(".id") => {
                ("structure".to_string(), ErrorSeverity::Critical, 1.0)
            }
            c if c.starts_with("structure.timestamp") => {
                ("structure".to_string(), ErrorSeverity::High, 0.9)
            }
            c if c.starts_with("structure.kind") => {
                ("structure".to_string(), ErrorSeverity::Critical, 1.0)
            }
            c if c.starts_with("structure.text") => {
                ("structure".to_string(), ErrorSeverity::High, 0.95)
            }
            c if c.starts_with("structure.importance") => {
                ("structure".to_string(), ErrorSeverity::Medium, 0.8)
            }
            c if c.starts_with("structure.tags") => {
                ("structure".to_string(), ErrorSeverity::Medium, 0.85)
            }
            c if c.starts_with("structure.metadata") => {
                ("structure".to_string(), ErrorSeverity::Low, 0.7)
            }
            c if c.starts_with("structure.embedding") => {
                ("embedding".to_string(), ErrorSeverity::Medium, 0.75)
            }
            c if c.starts_with("structure.source") => {
                ("audit".to_string(), ErrorSeverity::Low, 0.6)
            }
            c if c.starts_with("structure.checksum") => {
                ("integrity".to_string(), ErrorSeverity::Medium, 0.8)
            }
            c if c.starts_with("structure.references") => {
                ("integrity".to_string(), ErrorSeverity::High, 0.9)
            }
            c if c.starts_with("structure.required") => {
                ("structure".to_string(), ErrorSeverity::Critical, 1.0)
            }

            // Unknown codes
            _ => ("unknown".to_string(), ErrorSeverity::Medium, 0.5),
        }
    }
}

impl Default for MemoryErrorClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorClassifier<MemoryValidationEntry> for MemoryErrorClassifier {
    fn classify(
        &self,
        issue: &EccIssue,
        _context: &PipelineContext<MemoryValidationEntry>,
    ) -> MemoryEccResult<ErrorClassification> {
        Ok(self.classify_issue(issue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_critical_issues() {
        let classifier = MemoryErrorClassifier::new();

        let issue = EccIssue::new(
            "structure.memory_id_format".to_string(),
            "ID format invalid".to_string(),
            None,
            None,
        );

        let classification = classifier.classify_issue(&issue);
        assert_eq!(classification.severity, ErrorSeverity::Critical);
        assert_eq!(classification.confidence, 1.0);
    }

    #[test]
    fn test_classify_medium_issues() {
        let classifier = MemoryErrorClassifier::new();

        let issue = EccIssue::new(
            "structure.importance_out_of_range".to_string(),
            "Importance out of range".to_string(),
            None,
            None,
        );

        let classification = classifier.classify_issue(&issue);
        assert_eq!(classification.severity, ErrorSeverity::Medium);
    }

    #[test]
    fn test_classify_low_issues() {
        let classifier = MemoryErrorClassifier::new();

        let issue = EccIssue::new(
            "structure.source_invalid".to_string(),
            "Source invalid".to_string(),
            None,
            None,
        );

        let classification = classifier.classify_issue(&issue);
        assert_eq!(classification.severity, ErrorSeverity::Low);
    }
}
