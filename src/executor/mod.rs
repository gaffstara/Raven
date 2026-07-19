pub mod context;
pub mod engine;
pub mod report;
pub mod state;
#[cfg(test)]
mod tests;

use crate::error::RavenResult;
use crate::executor::context::ExecutorContext;
use crate::llm::Llm;
use crate::memory::MemoryStorage;
use crate::planner::ExecutionPlan;
use crate::tool::ToolManagerService;
use crate::EventBus;
use std::sync::{Arc, Mutex};

pub use context::ExecutorContext as ExecutorRuntimeContext;
pub use engine::ExecutorEngine;
pub use report::ExecutorReport;
pub use state::{ExecutorStepOutcome, ExecutorStepStatus};

/// Executor service interface.
pub trait Executor: Send + Sync {
    fn execute_plan(&self, plan: &ExecutionPlan) -> RavenResult<String>;
}

/// ExecutorService runs an ExecutionPlan using the production ExecutorEngine.
pub struct ExecutorService {
    engine: ExecutorEngine,
}

impl ExecutorService {
    pub fn new(
        memory: Arc<Mutex<Box<dyn MemoryStorage>>>,
        tools: Arc<Mutex<Box<dyn ToolManagerService>>>,
        llm: Arc<dyn Llm + Send + Sync>,
        planner: Arc<dyn crate::planner::PlannerProgress + Send + Sync>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            engine: ExecutorEngine::new(memory, tools, llm, planner, event_bus),
        }
    }

    /// Execute an entire plan and return a human-readable summary string.
    pub fn execute_plan(&self, plan: &ExecutionPlan) -> RavenResult<String> {
        let report = self
            .engine
            .execute_plan(plan, Some(ExecutorContext::new("executor")))?;
        Ok(report.summary())
    }

    /// Execute an entire plan and return a structured executor report.
    pub fn execute_plan_report(
        &self,
        plan: &ExecutionPlan,
        context: Option<ExecutorContext>,
    ) -> RavenResult<ExecutorReport> {
        self.engine.execute_plan(plan, context)
    }
}

impl Executor for ExecutorService {
    fn execute_plan(&self, plan: &ExecutionPlan) -> RavenResult<String> {
        self.execute_plan(plan)
    }
}
