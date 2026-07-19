//! Executor ECC validation rules.

use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::{ExecutorEccContext, ExecutorRequest};
use crate::ecc::report::EccIssue;

/// Trait that defines a validation rule for executor requests.
pub trait ExecutorRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn applies_to(&self, request: &ExecutorRequest, context: &ExecutorEccContext) -> bool;
    fn evaluate(
        &self,
        request: &ExecutorRequest,
        context: &ExecutorEccContext,
    ) -> EccResult<Vec<EccIssue>>;
}

/// Disallow unknown actions.
pub struct AllowedActionRule;

impl AllowedActionRule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AllowedActionRule {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutorRule for AllowedActionRule {
    fn id(&self) -> &'static str {
        "executor.action.allowed"
    }

    fn description(&self) -> &'static str {
        "Reject executor requests for actions that are not allowed."
    }

    fn applies_to(&self, _request: &ExecutorRequest, _context: &ExecutorEccContext) -> bool {
        true
    }

    fn evaluate(
        &self,
        request: &ExecutorRequest,
        context: &ExecutorEccContext,
    ) -> EccResult<Vec<EccIssue>> {
        if context.allows_action(&request.action) {
            Ok(Vec::new())
        } else {
            Ok(vec![EccIssue::new(
                self.id().to_string(),
                "executor.action.disallowed".to_string(),
                Some(format!("Action '{}' is not permitted.", request.action)),
                None,
            )])
        }
    }
}

/// Disallow unknown targets.
pub struct AllowedTargetRule;

impl AllowedTargetRule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AllowedTargetRule {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutorRule for AllowedTargetRule {
    fn id(&self) -> &'static str {
        "executor.target.allowed"
    }

    fn description(&self) -> &'static str {
        "Reject executor requests for targets that are not allowed."
    }

    fn applies_to(&self, _request: &ExecutorRequest, _context: &ExecutorEccContext) -> bool {
        true
    }

    fn evaluate(
        &self,
        request: &ExecutorRequest,
        context: &ExecutorEccContext,
    ) -> EccResult<Vec<EccIssue>> {
        if context.allows_target(&request.target) {
            Ok(Vec::new())
        } else {
            Ok(vec![EccIssue::new(
                self.id().to_string(),
                "executor.target.disallowed".to_string(),
                Some(format!("Target '{}' is not permitted.", request.target)),
                None,
            )])
        }
    }
}

/// Disallow denied parameter or target values.
pub struct DenyListRule;

impl DenyListRule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DenyListRule {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutorRule for DenyListRule {
    fn id(&self) -> &'static str {
        "executor.value.denied"
    }

    fn description(&self) -> &'static str {
        "Reject executor requests containing denied values in target or parameters."
    }

    fn applies_to(&self, _request: &ExecutorRequest, _context: &ExecutorEccContext) -> bool {
        true
    }

    fn evaluate(
        &self,
        request: &ExecutorRequest,
        context: &ExecutorEccContext,
    ) -> EccResult<Vec<EccIssue>> {
        let mut issues = Vec::new();

        if context.is_denied(&request.target) {
            issues.push(EccIssue::new(
                self.id().to_string(),
                "executor.target.denied".to_string(),
                Some(format!("Target '{}' is explicitly denied.", request.target)),
                None,
            ));
        }

        for (key, value) in &request.parameters {
            if context.is_denied(value) {
                issues.push(EccIssue::new(
                    self.id().to_string(),
                    "executor.parameter.denied".to_string(),
                    Some(format!("Parameter '{}' contains denied value.", key)),
                    None,
                ));
            }
        }

        Ok(issues)
    }
}

/// Validate parameter length limits.
pub struct ParameterLengthRule;

impl ParameterLengthRule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ParameterLengthRule {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutorRule for ParameterLengthRule {
    fn id(&self) -> &'static str {
        "executor.parameter.length"
    }

    fn description(&self) -> &'static str {
        "Reject executor requests with parameters exceeding configured length limits."
    }

    fn applies_to(&self, _request: &ExecutorRequest, _context: &ExecutorEccContext) -> bool {
        true
    }

    fn evaluate(
        &self,
        request: &ExecutorRequest,
        context: &ExecutorEccContext,
    ) -> EccResult<Vec<EccIssue>> {
        let mut issues = Vec::new();

        for (key, value) in &request.parameters {
            if value.len() > context.max_parameter_length {
                issues.push(EccIssue::new(
                    self.id().to_string(),
                    "executor.parameter.too_long".to_string(),
                    Some(format!(
                        "Parameter '{}' length {} exceeds maximum {}.",
                        key,
                        value.len(),
                        context.max_parameter_length
                    )),
                    None,
                ));
            }
        }

        Ok(issues)
    }
}
