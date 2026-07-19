use crate::ecc::executor::builder::ExecutorEccPipelineBuilder;
use crate::ecc::executor::types::{ExecutorEccContext, ExecutorRequest};

fn default_context() -> ExecutorEccContext {
    let mut context = ExecutorEccContext::new();
    context.allowed_actions = vec!["start".to_string(), "stop".to_string()];
    context.allowed_targets = vec!["vm-1".to_string(), "db-1".to_string()];
    context.deny_list = vec!["rm -rf /".to_string(), "shutdown".to_string()];
    context.max_parameter_length = 64;
    context
}

#[test]
fn executor_builder_builds_default_pipeline() {
    let context = default_context();
    let engine = ExecutorEccPipelineBuilder::new(context).build();
    let request = ExecutorRequest::new("req-1", "start", "vm-1")
        .with_parameters(std::collections::HashMap::new())
        .with_initiator("tester");

    let report = engine.execute(request).unwrap();
    assert!(report.validation_result.is_valid);
    assert_eq!(
        report.applied_action.action,
        crate::ecc::policy::PolicyAction::Accept
    );
}

#[test]
fn executor_disallowed_action_is_rejected() {
    let context = default_context();
    let engine = ExecutorEccPipelineBuilder::new(context).build();
    let request = ExecutorRequest::new("req-2", "delete", "vm-1")
        .with_parameters(std::collections::HashMap::new())
        .with_initiator("tester");

    let report = engine.execute(request).unwrap();
    assert!(!report.validation_result.is_valid);
    assert_eq!(
        report.applied_action.action,
        crate::ecc::policy::PolicyAction::Reject
    );
}

#[test]
fn executor_denied_target_value_is_rejected() {
    let mut context = default_context();
    context.deny_list.push("db-1".to_string());
    let engine = ExecutorEccPipelineBuilder::new(context).build();
    let request = ExecutorRequest::new("req-3", "start", "db-1")
        .with_parameters(std::collections::HashMap::new())
        .with_initiator("tester");

    let report = engine.execute(request).unwrap();
    assert!(!report.validation_result.is_valid);
    assert_eq!(
        report.applied_action.action,
        crate::ecc::policy::PolicyAction::Reject
    );
}

#[test]
fn executor_parameter_length_exceeding_limit_is_rejected() {
    let mut context = default_context();
    context.max_parameter_length = 3;
    let mut parameters = std::collections::HashMap::new();
    parameters.insert("payload".to_string(), "abcd".to_string());

    let engine = ExecutorEccPipelineBuilder::new(context).build();
    let request = ExecutorRequest::new("req-4", "start", "vm-1")
        .with_parameters(parameters)
        .with_initiator("tester");

    let report = engine.execute(request).unwrap();
    assert!(!report.validation_result.is_valid);
    assert_eq!(
        report.applied_action.action,
        crate::ecc::policy::PolicyAction::Reject
    );
}
