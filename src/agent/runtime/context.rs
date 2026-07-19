use crate::planner::ExecutionPlan;
use serde_json::Value;
use std::sync::Arc;

/// RuntimeContext is built for each execution and is immutable afterwards.
#[derive(Debug, Clone)]
pub struct RuntimeContext {
    pub session_id: String,
    pub conversation_id: Option<String>,
    pub user_input: String,
    pub plan: Option<ExecutionPlan>,
    pub metadata: Arc<Vec<(String, Value)>>,
}

impl RuntimeContext {
    pub fn new(session_id: impl Into<String>, user_input: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            conversation_id: None,
            user_input: user_input.into(),
            plan: None,
            metadata: Arc::new(Vec::new()),
        }
    }

    pub fn with_plan(mut self, plan: ExecutionPlan) -> Self {
        self.plan = Some(plan);
        self
    }
}
