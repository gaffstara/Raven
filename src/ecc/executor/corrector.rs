use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::ExecutorRequest;
use crate::ecc::report::ValidationReport;
use crate::ecc::traits::Corrector;

/// Executor request corrector that normalizes action, target, and parameter values.
pub struct ExecutorCorrector;

impl ExecutorCorrector {
    /// Create a new executor corrector.
    pub fn new() -> Self {
        Self
    }

    fn normalize_text(&self, input: &str) -> String {
        input.trim().to_lowercase()
    }

    fn sanitize_parameters(
        &self,
        parameters: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, String> {
        parameters
            .iter()
            .filter_map(|(key, value)| {
                let normalized = value.trim().to_string();
                if normalized.is_empty() {
                    None
                } else {
                    Some((key.clone(), normalized))
                }
            })
            .collect()
    }
}

impl Default for ExecutorCorrector {
    fn default() -> Self {
        Self::new()
    }
}

impl Corrector<ExecutorRequest> for ExecutorCorrector {
    fn correct(
        &self,
        subject: &ExecutorRequest,
        _report: &ValidationReport,
    ) -> EccResult<ExecutorRequest> {
        let corrected_action = self.normalize_text(&subject.action);
        let corrected_target = self.normalize_text(&subject.target);
        let corrected_parameters = self.sanitize_parameters(&subject.parameters);

        Ok(ExecutorRequest {
            id: subject.id.clone(),
            action: corrected_action,
            target: corrected_target,
            parameters: corrected_parameters,
            initiator: subject.initiator.clone(),
            timestamp: subject.timestamp,
        })
    }
}
