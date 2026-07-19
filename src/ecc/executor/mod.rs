//! Executor ECC validation, correction, and policy support.

pub mod builder;
pub mod classifier;
pub mod confidence;
pub mod context;
pub mod corrector;
pub mod engine;
pub mod errors;
pub mod pipeline;
pub mod policy;
pub mod report;
pub mod rules;
#[cfg(test)]
pub mod tests;
pub mod types;
pub mod validator;

pub use builder::ExecutorEccPipelineBuilder;
pub use classifier::ExecutorErrorClassifier;
pub use confidence::ExecutorConfidenceScorer;
pub use corrector::ExecutorCorrector;
pub use engine::ExecutorEccEngine;
pub use policy::ExecutorPolicy;
pub use report::ExecutorReporter;
pub use rules::{
    AllowedActionRule, AllowedTargetRule, DenyListRule, ExecutorRule, ParameterLengthRule,
};
pub use types::{ExecutorEccContext, ExecutorRequest};
