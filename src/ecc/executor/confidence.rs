use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::ExecutorRequest;
use crate::ecc::pipeline::PipelineContext;
use crate::ecc::report::ConfidenceScore;
use crate::ecc::traits::ConfidenceScorer;

/// Confidence scorer for Executor ECC results.
pub struct ExecutorConfidenceScorer;

impl ExecutorConfidenceScorer {
    /// Create a new executor confidence scorer.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ExecutorConfidenceScorer {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfidenceScorer<ExecutorRequest> for ExecutorConfidenceScorer {
    fn score(&self, context: &PipelineContext<ExecutorRequest>) -> EccResult<ConfidenceScore> {
        let report = context.validation_report.as_ref().ok_or_else(|| {
            crate::ecc::errors::EccError::Pipeline {
                details: "missing validation report for confidence scoring".into(),
            }
        })?;

        let mut score = 1.0_f32;
        if !report.is_valid {
            let issue_count = report.issues.len() as f32;
            score -= (issue_count * 0.12).min(0.85);
        }

        if !context.applied_fixes.is_empty() {
            score -= 0.04;
        }

        let rationale = Some(format!(
            "{} issue(s), {} fix(es) applied",
            report.issues.len(),
            context.applied_fixes.len()
        ));

        Ok(ConfidenceScore::new(score.max(0.0), rationale))
    }
}
