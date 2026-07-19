//! Context shared across executor ECC validation and correction.

use crate::ecc::executor::types::ExecutorEccContext;

/// Wrapper for executor ECC context.
#[derive(Debug, Clone)]
pub struct ExecutorEccContextWrapper {
    pub executor_context: ExecutorEccContext,
}

impl ExecutorEccContextWrapper {
    pub fn new(executor_context: ExecutorEccContext) -> Self {
        Self { executor_context }
    }
}
