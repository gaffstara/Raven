use crate::agent::runtime::metrics::RuntimeMetrics;
use crate::agent::runtime::state::LifecycleState;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RuntimeReport {
    pub status: LifecycleState,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub metrics: RuntimeMetrics,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl RuntimeReport {
    pub fn new(status: LifecycleState) -> Self {
        Self {
            status,
            started_at: Utc::now(),
            finished_at: None,
            metrics: RuntimeMetrics::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn complete(&mut self) {
        self.finished_at = Some(Utc::now());
    }
}
