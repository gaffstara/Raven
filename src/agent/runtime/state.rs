use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleState {
    Idle,
    ReceivingInput,
    UnderstandingIntent,
    Planning,
    MemoryLookup,
    KnowledgeLookup,
    WorkflowCreation,
    WorkflowValidation,
    ExecutionValidation,
    Execution,
    Reflection,
    MemoryUpdate,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct SessionState {
    pub state: LifecycleState,
    pub updated_at: DateTime<Utc>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Idle,
            updated_at: Utc::now(),
        }
    }
    pub fn set(&mut self, s: LifecycleState) {
        self.state = s;
        self.updated_at = Utc::now();
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}
