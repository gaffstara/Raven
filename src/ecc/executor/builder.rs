use crate::ecc::executor::classifier::ExecutorErrorClassifier;
use crate::ecc::executor::confidence::ExecutorConfidenceScorer;
use crate::ecc::executor::corrector::ExecutorCorrector;
use crate::ecc::executor::engine::ExecutorEccEngine;
use crate::ecc::executor::pipeline::ExecutorEccPipeline;
use crate::ecc::executor::policy::ExecutorPolicy;
use crate::ecc::executor::report::ExecutorReporter;
use crate::ecc::executor::rules::{
    AllowedActionRule, AllowedTargetRule, DenyListRule, ExecutorRule, ParameterLengthRule,
};
use crate::ecc::executor::types::ExecutorEccContext;
use crate::ecc::executor::validator::ExecutorValidator;
use std::sync::Arc;

/// Builder for an executor ECC engine.
pub struct ExecutorEccPipelineBuilder {
    context: ExecutorEccContext,
    rules: Vec<Box<dyn ExecutorRule>>,
}

impl ExecutorEccPipelineBuilder {
    /// Create a new builder with default rule set.
    pub fn new(context: ExecutorEccContext) -> Self {
        Self {
            context,
            rules: vec![
                Box::new(AllowedActionRule::new()),
                Box::new(AllowedTargetRule::new()),
                Box::new(DenyListRule::new()),
                Box::new(ParameterLengthRule::new()),
            ],
        }
    }

    /// Customize the rule set used by the pipeline.
    pub fn with_rules(mut self, rules: Vec<Box<dyn ExecutorRule>>) -> Self {
        self.rules = rules;
        self
    }

    /// Build a production ready executor ECC engine.
    pub fn build(self) -> ExecutorEccEngine {
        let validator = Arc::new(ExecutorValidator::new(
            crate::ecc::executor::context::ExecutorEccContextWrapper::new(self.context.clone()),
            self.rules,
        ));
        let corrector = Arc::new(ExecutorCorrector::new());
        let classifier = Arc::new(ExecutorErrorClassifier);
        let policy = Arc::new(ExecutorPolicy);
        let scorer = Arc::new(ExecutorConfidenceScorer);
        let reporter = Arc::new(ExecutorReporter::new());

        let pipeline = ExecutorEccPipeline::build(
            Box::new(
                crate::ecc::executor::pipeline::ExecutorValidationStage::new(validator.clone()),
            ),
            Box::new(
                crate::ecc::executor::pipeline::ExecutorCorrectionStage::new(corrector.clone()),
            ),
            Box::new(
                crate::ecc::executor::pipeline::ExecutorClassificationStage::new(
                    classifier.clone(),
                ),
            ),
            Box::new(crate::ecc::executor::pipeline::ExecutorPolicyStage::new(
                policy.clone(),
            )),
            Box::new(crate::ecc::executor::pipeline::ExecutorConfidenceStage::new(scorer.clone())),
            Box::new(crate::ecc::executor::pipeline::ExecutorReportingStage::new(
                reporter.clone(),
            )),
        );

        ExecutorEccEngine::new(
            validator,
            corrector,
            classifier,
            scorer,
            reporter,
            policy,
            pipeline,
            self.context,
        )
    }
}
