use crate::ecc::policy::{PolicyAction, PolicyDecision};
use crate::ecc::report::ValidationReport;
use crate::ecc::traits::Policy;

/// Simple policy for executor ECC decisions.
pub struct ExecutorPolicy;

impl ExecutorPolicy {
    /// Create a new executor policy.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ExecutorPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy for ExecutorPolicy {
    fn decide(&self, report: &ValidationReport) -> PolicyDecision {
        if report.is_valid {
            PolicyDecision::accept()
        } else {
            PolicyDecision::new(
                PolicyAction::Reject,
                Some("executor request rejected due to validation failures".into()),
            )
        }
    }
}
