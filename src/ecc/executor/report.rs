//! Reporting helpers for executor ECC.

use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::ExecutorRequest;
use crate::ecc::pipeline::PipelineContext;
use crate::ecc::report::EccReport;
use crate::ecc::traits::Reporter;

/// Executor ECC reporter that builds the final ECC report.
pub struct ExecutorReporter;

impl ExecutorReporter {
    /// Create a new executor reporter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ExecutorReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter<ExecutorRequest> for ExecutorReporter {
    fn generate(&self, context: &PipelineContext<ExecutorRequest>) -> EccResult<EccReport> {
        let validation =
            context
                .validation_report
                .clone()
                .ok_or(crate::ecc::errors::EccError::Reporting {
                    details: "missing validation report".into(),
                })?;

        let score =
            context
                .confidence_score
                .clone()
                .ok_or(crate::ecc::errors::EccError::Reporting {
                    details: "missing confidence score".into(),
                })?;

        let action =
            context
                .applied_action
                .clone()
                .ok_or(crate::ecc::errors::EccError::Reporting {
                    details: "missing policy decision".into(),
                })?;

        Ok(EccReport::new(
            validation,
            context.error_classification.clone(),
            score,
            action,
            context.executed_rules.clone(),
            context.applied_fixes.clone(),
            std::time::Instant::now().elapsed(),
            chrono::Utc::now(),
        ))
    }
}
