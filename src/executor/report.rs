use crate::executor::state::ExecutorStepOutcome;
use std::time::Duration;

/// Report emitted after executor runtime completes an execution plan.
#[derive(Debug, Clone)]
pub struct ExecutorReport {
    pub step_outcomes: Vec<ExecutorStepOutcome>,
    pub total_steps: usize,
    pub completed_steps: usize,
    pub failed_steps: usize,
    pub skipped_steps: usize,
    pub duration: Duration,
}

impl ExecutorReport {
    pub fn new(step_outcomes: Vec<ExecutorStepOutcome>, duration: Duration) -> Self {
        let total_steps = step_outcomes.len();
        let completed_steps = step_outcomes
            .iter()
            .filter(|item| item.status == crate::executor::state::ExecutorStepStatus::Completed)
            .count();
        let failed_steps = step_outcomes
            .iter()
            .filter(|item| item.status == crate::executor::state::ExecutorStepStatus::Failed)
            .count();
        let skipped_steps = step_outcomes
            .iter()
            .filter(|item| item.status == crate::executor::state::ExecutorStepStatus::Skipped)
            .count();

        Self {
            step_outcomes,
            total_steps,
            completed_steps,
            failed_steps,
            skipped_steps,
            duration,
        }
    }

    pub fn is_success(&self) -> bool {
        self.failed_steps == 0
    }

    pub fn summary(&self) -> String {
        let items: Vec<String> = self
            .step_outcomes
            .iter()
            .map(|outcome| {
                let status = match outcome.status {
                    crate::executor::state::ExecutorStepStatus::Completed => "completed",
                    crate::executor::state::ExecutorStepStatus::Failed => "failed",
                    crate::executor::state::ExecutorStepStatus::Skipped => "skipped",
                    crate::executor::state::ExecutorStepStatus::InProgress => "in-progress",
                    crate::executor::state::ExecutorStepStatus::Pending => "pending",
                };
                let details = if let Some(output) = &outcome.output {
                    format!("output={}", output)
                } else if let Some(error) = &outcome.error {
                    format!("error={}", error)
                } else {
                    "no result".to_string()
                };
                format!("{}:{}:{}", outcome.step_id, status, details)
            })
            .collect();

        format!(
            "ExecutorReport total={} completed={} failed={} skipped={} duration_ms={} steps=[{}]",
            self.total_steps,
            self.completed_steps,
            self.failed_steps,
            self.skipped_steps,
            self.duration.as_millis(),
            items.join(", ")
        )
    }
}
