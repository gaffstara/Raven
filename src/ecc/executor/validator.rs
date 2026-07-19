use crate::ecc::errors::EccResult;
use crate::ecc::executor::context::ExecutorEccContextWrapper;
use crate::ecc::executor::rules::ExecutorRule;
use crate::ecc::executor::types::ExecutorRequest;
use crate::ecc::report::ValidationReport;
use crate::ecc::traits::Validator;
use chrono::Utc;
use std::time::Instant;

/// Rule-based validator for executor requests.
pub struct ExecutorValidator {
    context: ExecutorEccContextWrapper,
    rules: Vec<Box<dyn ExecutorRule>>,
}

impl ExecutorValidator {
    /// Create a new executor validator.
    pub fn new(context: ExecutorEccContextWrapper, rules: Vec<Box<dyn ExecutorRule>>) -> Self {
        Self { context, rules }
    }
}

impl Validator<ExecutorRequest> for ExecutorValidator {
    fn validate(&self, subject: &ExecutorRequest) -> EccResult<ValidationReport> {
        let start = Instant::now();
        let mut issues = Vec::new();

        for rule in &self.rules {
            if rule.applies_to(subject, &self.context.executor_context) {
                let mut rule_issues = rule.evaluate(subject, &self.context.executor_context)?;
                issues.append(&mut rule_issues);
            }
        }

        let report = ValidationReport::new(Utc::now(), start.elapsed(), issues);
        Ok(report)
    }

    fn rule_ids(&self) -> Vec<&'static str> {
        self.rules.iter().map(|rule| rule.id()).collect()
    }
}
