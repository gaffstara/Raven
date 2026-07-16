//! Memory entry policy for post-validation decision making.

use crate::ecc::policy::{Policy, PolicyAction, PolicyDecision};
use crate::ecc::report::ValidationReport;

/// Policy for memory entries after validation.
///
/// This policy determines what action to take based on validation results:
/// - Accept: Use entry as-is (all valid)
/// - Correct: Apply corrections and use
/// - Retry: Temporary issue, retry later
/// - Reject: Don't use, log issue
/// - Abort: Critical error, stop processing
pub struct MemoryPolicy;

impl MemoryPolicy {
    /// Create a new memory policy.
    pub fn new() -> Self {
        Self
    }

    /// Determine the appropriate action based on validation results.
    fn determine_action(&self, report: &ValidationReport) -> (PolicyAction, String) {
        if report.is_valid {
            // No issues, accept as-is
            return (
                PolicyAction::Accept,
                "All validation checks passed".to_string(),
            );
        }

        // Count issues by severity
        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;

        for issue in &report.issues {
            match issue.code.as_str() {
                // Critical issues
                c if c.contains("memory_id")
                    || c.contains("required")
                    || c.contains("kind")
                    || c.contains("text_not_empty") =>
                {
                    critical_count += 1;
                }

                // High severity
                c if c.contains("timestamp") || c.contains("text_max") => {
                    high_count += 1;
                }

                // Medium severity
                c if c.contains("importance")
                    || c.contains("tags")
                    || c.contains("embedding")
                    || c.contains("checksum") =>
                {
                    medium_count += 1;
                }

                // Low severity
                _ => low_count += 1,
            }
        }

        // Decision logic
        if critical_count > 0 {
            // Cannot fix critical issues
            (
                PolicyAction::Reject,
                format!(
                    "Found {} critical issues that cannot be automatically corrected",
                    critical_count
                ),
            )
        } else if high_count > 0 {
            // High severity issues should be corrected if possible
            (
                PolicyAction::Correct,
                format!(
                    "Found {} high-severity issues, will attempt correction",
                    high_count
                ),
            )
        } else if medium_count > 0 || low_count > 0 {
            // Medium and low issues can usually be corrected
            (
                PolicyAction::Correct,
                format!(
                    "Found {} medium and {} low severity issues",
                    medium_count, low_count
                ),
            )
        } else {
            // Fallback
            (
                PolicyAction::Accept,
                "No recoverable issues found".to_string(),
            )
        }
    }
}

impl Default for MemoryPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy for MemoryPolicy {
    fn decide(&self, report: &ValidationReport) -> PolicyDecision {
        let (action, rationale) = self.determine_action(report);
        PolicyDecision::new(action, Some(rationale))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecc::report::EccIssue;
    use chrono::Utc;

    #[test]
    fn test_policy_accepts_valid_entry() {
        let policy = MemoryPolicy::new();
        let report = ValidationReport::new(Utc::now(), std::time::Duration::from_secs(0), vec![]);

        let decision = policy.decide(&report);
        assert_eq!(decision.action, PolicyAction::Accept);
    }

    #[test]
    fn test_policy_corrects_medium_issues() {
        let policy = MemoryPolicy::new();
        let issues = vec![EccIssue::new(
            "structure.importance_out_of_range".to_string(),
            "Issue".to_string(),
            None,
            None,
        )];

        let report = ValidationReport::new(Utc::now(), std::time::Duration::from_secs(0), issues);

        let decision = policy.decide(&report);
        assert_eq!(decision.action, PolicyAction::Correct);
    }

    #[test]
    fn test_policy_rejects_critical_issues() {
        let policy = MemoryPolicy::new();
        let issues = vec![EccIssue::new(
            "structure.memory_id_format".to_string(),
            "Issue".to_string(),
            None,
            None,
        )];

        let report = ValidationReport::new(Utc::now(), std::time::Duration::from_secs(0), issues);

        let decision = policy.decide(&report);
        assert_eq!(decision.action, PolicyAction::Reject);
    }
}
