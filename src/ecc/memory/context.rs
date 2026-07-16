//! Pipeline context for Memory ECC processing.

use crate::ecc::memory::types::MemoryValidationEntry;
use crate::ecc::policy::PolicyDecision;
use crate::ecc::report::{ConfidenceScore, ErrorClassification, ValidationReport};

/// Context that carries state through the Memory ECC pipeline.
///
/// Each stage in the pipeline populates different fields as the entry
/// is validated, corrected, classified, and scored.
#[derive(Clone)]
pub struct MemoryEccContext {
    /// The original memory entry being validated
    pub subject: MemoryValidationEntry,

    /// Validation report from the validation stage
    pub validation_report: Option<ValidationReport>,

    /// The corrected version of the entry after correction stage
    pub corrected_subject: Option<MemoryValidationEntry>,

    /// Classifications for each issue found
    pub error_classifications: Vec<ErrorClassification>,

    /// Overall confidence score for this entry
    pub confidence_score: Option<ConfidenceScore>,

    /// Policy decision (Accept, Correct, Reject, etc.)
    pub policy_decision: Option<PolicyDecision>,

    /// List of rule IDs that were executed
    pub executed_rules: Vec<String>,

    /// List of corrections that were applied
    pub applied_corrections: Vec<String>,

    /// Flag indicating if entry passed all validations
    pub is_valid: bool,

    /// Flag indicating if entry was corrected
    pub was_corrected: bool,
}

impl MemoryEccContext {
    /// Create a new empty context for a memory entry.
    pub fn new(subject: MemoryValidationEntry) -> Self {
        Self {
            subject,
            validation_report: None,
            corrected_subject: None,
            error_classifications: Vec::new(),
            confidence_score: None,
            policy_decision: None,
            executed_rules: Vec::new(),
            applied_corrections: Vec::new(),
            is_valid: false,
            was_corrected: false,
        }
    }

    /// Mark this context as having passed validation.
    pub fn mark_valid(&mut self) {
        self.is_valid = true;
    }

    /// Mark this context as having been corrected.
    pub fn mark_corrected(&mut self, corrected_subject: MemoryValidationEntry) {
        self.corrected_subject = Some(corrected_subject);
        self.was_corrected = true;
    }

    /// Record that a rule was executed.
    pub fn record_rule(&mut self, rule_id: String) {
        self.executed_rules.push(rule_id);
    }

    /// Record that a correction was applied.
    pub fn record_correction(&mut self, correction: String) {
        self.applied_corrections.push(correction);
    }

    /// Get the effective subject (corrected if available, otherwise original).
    pub fn effective_subject(&self) -> &MemoryValidationEntry {
        self.corrected_subject.as_ref().unwrap_or(&self.subject)
    }

    /// Check if validation report exists and is valid.
    pub fn is_validation_report_present(&self) -> bool {
        self.validation_report.is_some()
    }

    /// Check if classification data exists.
    pub fn is_classification_present(&self) -> bool {
        !self.error_classifications.is_empty()
    }

    /// Check if scoring data exists.
    pub fn is_scoring_present(&self) -> bool {
        self.confidence_score.is_some()
    }

    /// Check if policy decision exists.
    pub fn is_policy_present(&self) -> bool {
        self.policy_decision.is_some()
    }

    /// Get total number of issues from validation report.
    pub fn issue_count(&self) -> usize {
        self.validation_report
            .as_ref()
            .map(|r| r.issues.len())
            .unwrap_or(0)
    }

    /// Get total number of applied corrections.
    pub fn correction_count(&self) -> usize {
        self.applied_corrections.len()
    }
}
