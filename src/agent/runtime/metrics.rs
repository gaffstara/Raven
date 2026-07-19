use std::time::Duration;

#[derive(Debug, Default, Clone)]
pub struct RuntimeMetrics {
    pub execution_duration: Option<Duration>,
    pub planner_duration: Option<Duration>,
    pub memory_duration: Option<Duration>,
    pub workflow_duration: Option<Duration>,
    pub executor_duration: Option<Duration>,
    pub retry_count: u32,
    pub failure_count: u32,
}

impl RuntimeMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
