use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::ExecutorRequest;
use crate::ecc::pipeline::PipelineContext;
use crate::ecc::report::{ErrorClassification, ErrorSeverity};
use crate::ecc::traits::ErrorClassifier;

/// Classifier for executor request validation issues.
pub struct ExecutorErrorClassifier;

impl ExecutorErrorClassifier {
    /// Create a new classifier.
    pub fn new() -> Self {
        Self
    }

    fn classify_issue(&self, code: &str) -> (String, ErrorSeverity, f32) {
        let lower = code.to_lowercase();
        if lower.contains("disallowed") || lower.contains("denied") {
            ("security".to_string(), ErrorSeverity::Critical, 1.0)
        } else if lower.contains("too_long") {
            ("validation".to_string(), ErrorSeverity::High, 0.9)
        } else if lower.contains("allowed") {
            ("validation".to_string(), ErrorSeverity::Medium, 0.75)
        } else {
            ("general".to_string(), ErrorSeverity::Low, 0.6)
        }
    }
}

impl Default for ExecutorErrorClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorClassifier<ExecutorRequest> for ExecutorErrorClassifier {
    fn classify(
        &self,
        issue: &crate::ecc::report::EccIssue,
        _context: &PipelineContext<ExecutorRequest>,
    ) -> EccResult<ErrorClassification> {
        let (category, severity, confidence) = self.classify_issue(&issue.code);
        Ok(ErrorClassification {
            issue_code: issue.code.clone(),
            category,
            severity,
            confidence,
        })
    }
}
