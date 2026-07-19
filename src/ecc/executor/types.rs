//! Executor ECC domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Executor action request submitted for validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutorRequest {
    pub id: String,
    pub action: String,
    pub target: String,
    pub parameters: HashMap<String, String>,
    pub initiator: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ExecutorRequest {
    pub fn new(
        id: impl Into<String>,
        action: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            action: action.into(),
            target: target.into(),
            parameters: HashMap::new(),
            initiator: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_parameters(mut self, parameters: HashMap<String, String>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_initiator(mut self, initiator: impl Into<String>) -> Self {
        self.initiator = Some(initiator.into());
        self
    }
}

/// Execution metadata context available during ECC processing.
#[derive(Debug, Clone)]
pub struct ExecutorEccContext {
    pub allowed_actions: Vec<String>,
    pub allowed_targets: Vec<String>,
    pub deny_list: Vec<String>,
    pub max_parameter_length: usize,
}

impl ExecutorEccContext {
    pub fn new() -> Self {
        Self {
            allowed_actions: Vec::new(),
            allowed_targets: Vec::new(),
            deny_list: Vec::new(),
            max_parameter_length: 256,
        }
    }

    pub fn allows_action(&self, action: &str) -> bool {
        self.allowed_actions.is_empty() || self.allowed_actions.iter().any(|a| a == action)
    }

    pub fn allows_target(&self, target: &str) -> bool {
        self.allowed_targets.is_empty() || self.allowed_targets.iter().any(|t| t == target)
    }

    pub fn is_denied(&self, value: &str) -> bool {
        self.deny_list.iter().any(|item| item == value)
    }
}

impl Default for ExecutorEccContext {
    fn default() -> Self {
        Self::new()
    }
}
