//! Memory ECC report generation.

use crate::ecc::report::EccReport;
use chrono::{DateTime, Utc};
use std::time::Duration;

/// Comprehensive Memory ECC report.
///
/// This report is generated at the end of the Memory ECC pipeline
/// and provides detailed information about validation, correction,
/// classification, and decision-making.
#[derive(Debug, Clone)]
pub struct MemoryEccReport {
    /// The underlying ECC report
    pub ecc_report: EccReport,

    /// Memory-specific metadata
    pub memory_id: String,

    /// Whether the memory entry was ultimately accepted
    pub accepted: bool,

    /// Whether corrections were applied
    pub corrected: bool,

    /// Timestamp of report generation
    pub generated_at: DateTime<Utc>,

    /// Processing duration
    pub duration: Duration,
}

impl MemoryEccReport {
    /// Create a new memory ECC report.
    pub fn new(ecc_report: EccReport, memory_id: String, accepted: bool, corrected: bool) -> Self {
        let duration = ecc_report.duration;
        let generated_at = ecc_report.timestamp;

        Self {
            ecc_report,
            memory_id,
            accepted,
            corrected,
            generated_at,
            duration,
        }
    }

    /// Get a summary of this report as a string.
    pub fn summary(&self) -> String {
        let status = if self.accepted {
            if self.corrected {
                "ACCEPTED_WITH_CORRECTION"
            } else {
                "ACCEPTED"
            }
        } else {
            "REJECTED"
        };

        format!(
            "MemoryEcc Report [{}] - ID: {} | Issues: {} | Confidence: {:.2} | Duration: {:?}",
            status,
            self.memory_id,
            self.ecc_report.validation_result.issues.len(),
            self.ecc_report.confidence_score.value,
            self.duration
        )
    }

    /// Check if the entry passed validation without issues.
    pub fn is_valid(&self) -> bool {
        self.ecc_report.validation_result.is_valid
    }

    /// Get the number of issues found.
    pub fn issue_count(&self) -> usize {
        self.ecc_report.validation_result.issues.len()
    }

    /// Get the confidence score value.
    pub fn confidence(&self) -> f32 {
        self.ecc_report.confidence_score.value
    }

    /// Get the number of corrections applied.
    pub fn correction_count(&self) -> usize {
        self.ecc_report.applied_fixes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecc::policy::PolicyDecision;
    use crate::ecc::report::{ConfidenceScore, ValidationReport};

    #[test]
    fn test_report_creation() {
        let validation = ValidationReport::new(Utc::now(), Duration::from_millis(10), vec![]);

        let confidence = ConfidenceScore::new(0.95, None);
        let policy = PolicyDecision::accept();

        let ecc_report = EccReport::new(
            validation,
            vec![],
            confidence,
            policy,
            vec![],
            vec![],
            Duration::from_millis(10),
            Utc::now(),
        );

        let memory_report = MemoryEccReport::new(ecc_report, "m00000001".to_string(), true, false);

        assert!(memory_report.is_valid());
        assert_eq!(memory_report.issue_count(), 0);
        assert!(memory_report.confidence() > 0.9);
    }

    #[test]
    fn test_report_summary() {
        let validation = ValidationReport::new(Utc::now(), Duration::from_millis(10), vec![]);

        let confidence = ConfidenceScore::new(0.95, None);
        let policy = PolicyDecision::accept();

        let ecc_report = EccReport::new(
            validation,
            vec![],
            confidence,
            policy,
            vec![],
            vec![],
            Duration::from_millis(10),
            Utc::now(),
        );

        let memory_report = MemoryEccReport::new(ecc_report, "m00000001".to_string(), true, false);

        let summary = memory_report.summary();
        assert!(summary.contains("ACCEPTED"));
        assert!(summary.contains("m00000001"));
    }
}
