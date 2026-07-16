//! Memory ECC pipeline with staged processing.

use crate::ecc::memory::classifier::MemoryErrorClassifier;
use crate::ecc::memory::corrector::MemoryCorrector;
use crate::ecc::memory::errors::MemoryEccResult;
use crate::ecc::memory::policy::MemoryPolicy;
use crate::ecc::memory::scorer::MemoryConfidenceScorer;
use crate::ecc::memory::types::MemoryValidationEntry;
use crate::ecc::memory::validator::MemoryValidator;
use crate::ecc::pipeline::PipelineContext;
use crate::ecc::policy::Policy;
use crate::ecc::policy::PolicyAction;
use crate::ecc::report::EccReport;
use crate::ecc::traits::{ConfidenceScorer, Corrector, ErrorClassifier, PipelineStage, Validator};
use chrono::Utc;
use std::time::Instant;

/// Pipeline stage for memory validation.
struct MemoryValidationStage {
    validator: MemoryValidator,
}

impl MemoryValidationStage {
    fn new(validator: MemoryValidator) -> Self {
        Self { validator }
    }
}

impl PipelineStage<MemoryValidationEntry> for MemoryValidationStage {
    fn name(&self) -> &'static str {
        "memory_validation"
    }

    fn execute(&self, context: &mut PipelineContext<MemoryValidationEntry>) -> MemoryEccResult<()> {
        let report = self.validator.validate(&context.subject)?;
        context.validation_report = Some(report);
        Ok(())
    }
}

/// Pipeline stage for error classification.
struct MemoryClassificationStage {
    classifier: MemoryErrorClassifier,
}

impl MemoryClassificationStage {
    fn new(classifier: MemoryErrorClassifier) -> Self {
        Self { classifier }
    }
}

impl PipelineStage<MemoryValidationEntry> for MemoryClassificationStage {
    fn name(&self) -> &'static str {
        "memory_classification"
    }

    fn execute(&self, context: &mut PipelineContext<MemoryValidationEntry>) -> MemoryEccResult<()> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "validation report not found, run validation stage first".to_string(),
            }
        })?;

        // Classify each issue
        for issue in &report.issues {
            let classification = self.classifier.classify(issue, context)?;
            context.error_classification.push(classification);
        }

        Ok(())
    }
}

/// Pipeline stage for memory correction.
struct MemoryCorrectionStage {
    corrector: MemoryCorrector,
}

impl MemoryCorrectionStage {
    fn new(corrector: MemoryCorrector) -> Self {
        Self { corrector }
    }
}

impl PipelineStage<MemoryValidationEntry> for MemoryCorrectionStage {
    fn name(&self) -> &'static str {
        "memory_correction"
    }

    fn execute(&self, context: &mut PipelineContext<MemoryValidationEntry>) -> MemoryEccResult<()> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "validation report not found, run validation stage first".to_string(),
            }
        })?;

        if !report.is_valid {
            let corrected = self.corrector.correct(&context.subject, report)?;
            context.corrected_subject = Some(corrected);
        }

        Ok(())
    }
}

/// Pipeline stage for policy decision.
struct MemoryPolicyStage {
    policy: MemoryPolicy,
}

impl MemoryPolicyStage {
    fn new(policy: MemoryPolicy) -> Self {
        Self { policy }
    }
}

impl PipelineStage<MemoryValidationEntry> for MemoryPolicyStage {
    fn name(&self) -> &'static str {
        "memory_policy"
    }

    fn execute(&self, context: &mut PipelineContext<MemoryValidationEntry>) -> MemoryEccResult<()> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "validation report not found, run validation stage first".to_string(),
            }
        })?;

        let decision = self.policy.decide(report);
        context.applied_action = Some(decision);

        Ok(())
    }
}

/// Pipeline stage for confidence scoring.
struct MemoryScoringStage {
    scorer: MemoryConfidenceScorer,
}

impl MemoryScoringStage {
    fn new(scorer: MemoryConfidenceScorer) -> Self {
        Self { scorer }
    }
}

impl PipelineStage<MemoryValidationEntry> for MemoryScoringStage {
    fn name(&self) -> &'static str {
        "memory_scoring"
    }

    fn execute(&self, context: &mut PipelineContext<MemoryValidationEntry>) -> MemoryEccResult<()> {
        let score = self.scorer.score(context)?;
        context.confidence_score = Some(score);
        Ok(())
    }
}

/// Complete Memory ECC pipeline.
pub struct MemoryEccPipeline {
    stages: Vec<Box<dyn PipelineStage<MemoryValidationEntry>>>,
}

impl MemoryEccPipeline {
    /// Create a new pipeline with default stages.
    pub fn new() -> Self {
        let stages: Vec<Box<dyn PipelineStage<MemoryValidationEntry>>> = vec![
            Box::new(MemoryValidationStage::new(MemoryValidator::new())),
            Box::new(MemoryClassificationStage::new(MemoryErrorClassifier::new())),
            Box::new(MemoryCorrectionStage::new(MemoryCorrector::new())),
            Box::new(MemoryPolicyStage::new(MemoryPolicy::new())),
            Box::new(MemoryScoringStage::new(MemoryConfidenceScorer::new())),
        ];

        Self { stages }
    }

    /// Run the complete pipeline on a memory entry.
    pub fn run(
        &self,
        entry: MemoryValidationEntry,
    ) -> MemoryEccResult<(MemoryValidationEntry, EccReport)> {
        let start = Instant::now();
        let mut context = PipelineContext::new(entry);

        for stage in &self.stages {
            stage.execute(&mut context)?;
        }

        // Build final report
        let validation_report = context.validation_report.take().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "validation report missing".to_string(),
            }
        })?;

        let confidence_score = context.confidence_score.take().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "confidence score missing".to_string(),
            }
        })?;

        let policy_decision = context.applied_action.take().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "policy decision missing".to_string(),
            }
        })?;

        // Extract the action before moving policy_decision
        let final_action = policy_decision.action.clone();

        let report = EccReport::new(
            validation_report,
            context.error_classification.clone(),
            confidence_score,
            policy_decision,
            context.executed_rules.clone(),
            context.applied_fixes.clone(),
            start.elapsed(),
            Utc::now(),
        );

        // Determine final entry based on policy
        let final_entry = match final_action {
            PolicyAction::Accept => context.subject,
            PolicyAction::Correct => context.corrected_subject.unwrap_or(context.subject),
            PolicyAction::Retry | PolicyAction::Reject | PolicyAction::Abort => context.subject,
        };

        Ok((final_entry, report))
    }
}

impl Default for MemoryEccPipeline {
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
            text: "Test memory entry".to_string(),
            created_at: Utc::now(),
            tags: vec!["test".to_string()],
            importance: 0.5,
        };

        MemoryValidationEntry::from_entry(entry)
    }

    #[test]
    fn test_pipeline_with_valid_entry() {
        let pipeline = MemoryEccPipeline::new();
        let entry = create_valid_entry();

        let result = pipeline.run(entry);
        assert!(result.is_ok());

        let (_final_entry, report) = result.unwrap();
        assert!(report.validation_result.is_valid);
        assert_eq!(report.applied_action.action, PolicyAction::Accept);
    }

    #[test]
    fn test_pipeline_with_invalid_entry() {
        let pipeline = MemoryEccPipeline::new();
        let mut entry = create_valid_entry();
        entry.entry.id = "invalid".to_string();

        let result = pipeline.run(entry);
        assert!(result.is_ok());

        let (_final_entry, report) = result.unwrap();
        assert!(!report.validation_result.is_valid);
    }
}
