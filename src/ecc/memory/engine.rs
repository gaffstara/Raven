//! Memory ECC engine orchestration.

use crate::ecc::memory::classifier::MemoryErrorClassifier;
use crate::ecc::memory::corrector::MemoryCorrector;
use crate::ecc::memory::errors::MemoryEccResult;
use crate::ecc::memory::pipeline::MemoryEccPipeline;
use crate::ecc::memory::policy::MemoryPolicy;
use crate::ecc::memory::report::MemoryEccReport;
use crate::ecc::memory::scorer::MemoryConfidenceScorer;
use crate::ecc::memory::types::MemoryValidationEntry;
use crate::ecc::memory::validator::MemoryValidator;
use crate::ecc::policy::Policy;
use crate::ecc::traits::{ConfidenceScorer, Corrector, ErrorClassifier, Validator};

/// Main orchestrator for Memory ECC operations.
///
/// The MemoryEccEngine coordinates all components of the Memory ECC system:
/// - Validator: Detects invalid conditions via rules
/// - Corrector: Applies deterministic corrections
/// - Classifier: Categorizes errors by severity
/// - Scorer: Produces confidence scores
/// - Policy: Makes accept/reject/correct decisions
/// - Pipeline: Orchestrates all stages
///
/// This engine ensures memory entries are validated, corrected, classified,
/// scored, and validated again before being accepted for storage or retrieval.
#[allow(dead_code)]
pub struct MemoryEccEngine {
    validator: Box<dyn Validator<MemoryValidationEntry>>,
    corrector: Box<dyn Corrector<MemoryValidationEntry>>,
    classifier: Box<dyn ErrorClassifier<MemoryValidationEntry>>,
    scorer: Box<dyn ConfidenceScorer<MemoryValidationEntry>>,
    policy: Box<dyn Policy>,
    pipeline: MemoryEccPipeline,
}

impl MemoryEccEngine {
    /// Create a new Memory ECC engine with default components.
    pub fn new() -> Self {
        Self::with_components(
            Box::new(MemoryValidator::new()),
            Box::new(MemoryCorrector::new()),
            Box::new(MemoryErrorClassifier::new()),
            Box::new(MemoryConfidenceScorer::new()),
            Box::new(MemoryPolicy::new()),
            MemoryEccPipeline::new(),
        )
    }

    /// Create an engine with custom components.
    pub fn with_components(
        validator: Box<dyn Validator<MemoryValidationEntry>>,
        corrector: Box<dyn Corrector<MemoryValidationEntry>>,
        classifier: Box<dyn ErrorClassifier<MemoryValidationEntry>>,
        scorer: Box<dyn ConfidenceScorer<MemoryValidationEntry>>,
        policy: Box<dyn Policy>,
        pipeline: MemoryEccPipeline,
    ) -> Self {
        Self {
            validator,
            corrector,
            classifier,
            scorer,
            policy,
            pipeline,
        }
    }

    /// Execute the full Memory ECC pipeline on an entry.
    pub fn execute(
        &self,
        entry: MemoryValidationEntry,
    ) -> MemoryEccResult<(MemoryValidationEntry, MemoryEccReport)> {
        let memory_id = entry.entry.id.clone();

        // Run the full pipeline
        let (final_entry, ecc_report) = self.pipeline.run(entry)?;

        // Determine acceptance based on policy decision
        let accepted = matches!(
            ecc_report.applied_action.action,
            crate::ecc::policy::PolicyAction::Accept | crate::ecc::policy::PolicyAction::Correct
        );

        // Corrected is true if policy decided to correct the issues
        let corrected = matches!(
            ecc_report.applied_action.action,
            crate::ecc::policy::PolicyAction::Correct
        );

        let report = MemoryEccReport::new(ecc_report, memory_id, accepted, corrected);

        Ok((final_entry, report))
    }

    /// Validate an entry without correction.
    pub fn validate_only(
        &self,
        entry: &MemoryValidationEntry,
    ) -> MemoryEccResult<crate::ecc::report::ValidationReport> {
        self.validator.validate(entry)
    }

    /// Correct an entry based on a validation report.
    pub fn correct_only(
        &self,
        entry: &MemoryValidationEntry,
        report: &crate::ecc::report::ValidationReport,
    ) -> MemoryEccResult<MemoryValidationEntry> {
        self.corrector.correct(entry, report)
    }
}

impl Default for MemoryEccEngine {
    fn default() -> Self {
        Self::new()
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
            text: "Test memory".to_string(),
            created_at: Utc::now(),
            tags: vec!["test".to_string()],
            importance: 0.5,
        };

        MemoryValidationEntry::from_entry(entry)
    }

    fn create_invalid_entry() -> MemoryValidationEntry {
        let entry = MemoryEntry {
            id: "invalid_id".to_string(),
            kind: MemoryKind::Working,
            text: "".to_string(),
            created_at: Utc::now(),
            tags: vec![],
            importance: 1.5,
        };

        MemoryValidationEntry::from_entry(entry)
    }

    #[test]
    fn test_engine_processes_valid_entry() {
        let engine = MemoryEccEngine::new();
        let entry = create_valid_entry();

        let result = engine.execute(entry);
        assert!(result.is_ok());

        let (_final_entry, report) = result.unwrap();
        assert!(report.is_valid());
        assert!(report.accepted);
    }

    #[test]
    fn test_engine_processes_invalid_entry() {
        let engine = MemoryEccEngine::new();
        let entry = create_invalid_entry();

        let result = engine.execute(entry);
        assert!(result.is_ok());

        let (_final_entry, report) = result.unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn test_engine_validate_only() {
        let engine = MemoryEccEngine::new();
        let entry = create_valid_entry();

        let result = engine.validate_only(&entry);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(report.is_valid);
    }
}
