/// Status produced for each execution step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutorStepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Outcome summary for a single execution step.
#[derive(Debug, Clone)]
pub struct ExecutorStepOutcome {
    pub step_id: String,
    pub status: ExecutorStepStatus,
    pub attempts: u32,
    pub output: Option<String>,
    pub error: Option<String>,
    pub memory_id: Option<String>,
}

impl ExecutorStepOutcome {
    pub fn new(step_id: impl Into<String>) -> Self {
        Self {
            step_id: step_id.into(),
            status: ExecutorStepStatus::Pending,
            attempts: 0,
            output: None,
            error: None,
            memory_id: None,
        }
    }
}
