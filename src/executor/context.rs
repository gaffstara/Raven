use serde_json::Value;

/// Context available to the executor runtime and tools during execution.
#[derive(Debug, Clone)]
pub struct ExecutorContext {
    pub caller: String,
    pub workflow_id: Option<String>,
    pub metadata: Vec<(String, Value)>,
}

impl ExecutorContext {
    pub fn new(caller: impl Into<String>) -> Self {
        Self {
            caller: caller.into(),
            workflow_id: None,
            metadata: Vec::new(),
        }
    }

    pub fn with_workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    pub fn insert_metadata(&mut self, key: impl Into<String>, value: Value) {
        self.metadata.push((key.into(), value));
    }

    pub fn into_tool_context(self) -> crate::tool::ExecutionContext {
        let mut context = crate::tool::ExecutionContext::new().with_caller(self.caller);
        if let Some(workflow_id) = self.workflow_id {
            context.insert_metadata("workflow_id", Value::String(workflow_id));
        }
        for (key, value) in self.metadata {
            context.insert_metadata(key, value);
        }
        context
    }
}
