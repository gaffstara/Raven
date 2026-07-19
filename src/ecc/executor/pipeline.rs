//! Executor ECC pipeline stages.

use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::ExecutorRequest;
use crate::ecc::pipeline::{Pipeline, PipelineContext, PipelineStage};
use crate::ecc::traits::{Corrector, ErrorClassifier, Policy, Reporter, Validator};
use std::sync::Arc;

pub struct ExecutorValidationStage {
    validator: Arc<dyn Validator<ExecutorRequest>>,
}

impl ExecutorValidationStage {
    pub fn new(validator: Arc<dyn Validator<ExecutorRequest>>) -> Self {
        Self { validator }
    }
}

impl PipelineStage<ExecutorRequest> for ExecutorValidationStage {
    fn name(&self) -> &'static str {
        "executor_validation"
    }

    fn execute(&self, context: &mut PipelineContext<ExecutorRequest>) -> EccResult<()> {
        let report = self.validator.validate(&context.subject)?;
        context.executed_rules = self
            .validator
            .rule_ids()
            .into_iter()
            .map(|id| id.to_string())
            .collect();
        context.validation_report = Some(report);
        Ok(())
    }
}

pub struct ExecutorCorrectionStage {
    corrector: Arc<dyn Corrector<ExecutorRequest>>,
}

impl ExecutorCorrectionStage {
    pub fn new(corrector: Arc<dyn Corrector<ExecutorRequest>>) -> Self {
        Self { corrector }
    }
}

impl PipelineStage<ExecutorRequest> for ExecutorCorrectionStage {
    fn name(&self) -> &'static str {
        "executor_correction"
    }

    fn execute(&self, context: &mut PipelineContext<ExecutorRequest>) -> EccResult<()> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "missing validation report before correction".into(),
            }
        })?;

        let corrected = self.corrector.correct(&context.subject, report)?;
        if corrected != context.subject {
            context
                .applied_fixes
                .push("deterministic correction applied".into());
            context.corrected_subject = Some(corrected);
        }

        Ok(())
    }
}

pub struct ExecutorClassificationStage {
    classifier: Arc<dyn ErrorClassifier<ExecutorRequest>>,
}

impl ExecutorClassificationStage {
    pub fn new(classifier: Arc<dyn ErrorClassifier<ExecutorRequest>>) -> Self {
        Self { classifier }
    }
}

impl PipelineStage<ExecutorRequest> for ExecutorClassificationStage {
    fn name(&self) -> &'static str {
        "executor_classification"
    }

    fn execute(&self, context: &mut PipelineContext<ExecutorRequest>) -> EccResult<()> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "missing validation report before classification".into(),
            }
        })?;

        let mut classifications = Vec::new();
        for issue in &report.issues {
            classifications.push(self.classifier.classify(issue, context)?);
        }
        context.error_classification = classifications;
        Ok(())
    }
}

pub struct ExecutorPolicyStage {
    policy: Arc<dyn Policy>,
}

impl ExecutorPolicyStage {
    pub fn new(policy: Arc<dyn Policy>) -> Self {
        Self { policy }
    }
}

impl PipelineStage<ExecutorRequest> for ExecutorPolicyStage {
    fn name(&self) -> &'static str {
        "executor_policy"
    }

    fn execute(&self, context: &mut PipelineContext<ExecutorRequest>) -> EccResult<()> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "missing validation report before policy".into(),
            }
        })?;

        context.applied_action = Some(self.policy.decide(report));
        Ok(())
    }
}

pub struct ExecutorConfidenceStage {
    scorer: Arc<dyn crate::ecc::traits::ConfidenceScorer<ExecutorRequest>>,
}

impl ExecutorConfidenceStage {
    pub fn new(scorer: Arc<dyn crate::ecc::traits::ConfidenceScorer<ExecutorRequest>>) -> Self {
        Self { scorer }
    }
}

impl PipelineStage<ExecutorRequest> for ExecutorConfidenceStage {
    fn name(&self) -> &'static str {
        "executor_confidence"
    }

    fn execute(&self, context: &mut PipelineContext<ExecutorRequest>) -> EccResult<()> {
        let score = self.scorer.score(context)?;
        context.confidence_score = Some(score);
        Ok(())
    }
}

pub struct ExecutorReportingStage {
    reporter: Arc<dyn Reporter<ExecutorRequest>>,
}

impl ExecutorReportingStage {
    pub fn new(reporter: Arc<dyn Reporter<ExecutorRequest>>) -> Self {
        Self { reporter }
    }
}

impl PipelineStage<ExecutorRequest> for ExecutorReportingStage {
    fn name(&self) -> &'static str {
        "executor_reporting"
    }

    fn execute(&self, context: &mut PipelineContext<ExecutorRequest>) -> EccResult<()> {
        let report = self.reporter.generate(context)?;
        context.report = Some(report);
        Ok(())
    }
}

pub struct ExecutorEccPipeline;

impl ExecutorEccPipeline {
    pub fn build(
        validator_stage: Box<dyn PipelineStage<ExecutorRequest>>,
        corrector_stage: Box<dyn PipelineStage<ExecutorRequest>>,
        classifier_stage: Box<dyn PipelineStage<ExecutorRequest>>,
        policy_stage: Box<dyn PipelineStage<ExecutorRequest>>,
        confidence_stage: Box<dyn PipelineStage<ExecutorRequest>>,
        reporter_stage: Box<dyn PipelineStage<ExecutorRequest>>,
    ) -> Pipeline<ExecutorRequest> {
        Pipeline::new(vec![
            validator_stage,
            corrector_stage,
            classifier_stage,
            policy_stage,
            confidence_stage,
            reporter_stage,
        ])
    }
}
