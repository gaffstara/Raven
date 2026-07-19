use crate::error::{RavenError, RavenResult};
use crate::event::{AgentEvent, EventBus};
use crate::executor::context::ExecutorContext;
use crate::executor::report::ExecutorReport;
use crate::executor::state::{ExecutorStepOutcome, ExecutorStepStatus};
use crate::llm::Llm;
use crate::memory::{MemoryKind, MemoryStorage};
use crate::planner::{ExecutionPlan, PlannerProgress, Step};
use crate::tool::ToolManagerService;
use log::{error, info};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// The core runtime engine that executes a validated plan.
pub struct ExecutorEngine {
    memory: Arc<Mutex<Box<dyn MemoryStorage>>>,
    tools: Arc<Mutex<Box<dyn ToolManagerService>>>,
    llm: Arc<dyn Llm + Send + Sync>,
    planner: Arc<dyn PlannerProgress + Send + Sync>,
    event_bus: Arc<EventBus>,
}

impl ExecutorEngine {
    pub fn new(
        memory: Arc<Mutex<Box<dyn MemoryStorage>>>,
        tools: Arc<Mutex<Box<dyn ToolManagerService>>>,
        llm: Arc<dyn Llm + Send + Sync>,
        planner: Arc<dyn PlannerProgress + Send + Sync>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            memory,
            tools,
            llm,
            planner,
            event_bus,
        }
    }

    /// Execute an entire plan and produce a detailed execution report.
    pub fn execute_plan(
        &self,
        plan: &ExecutionPlan,
        context: Option<ExecutorContext>,
    ) -> RavenResult<ExecutorReport> {
        if plan.steps.is_empty() {
            return Err(RavenError::Executor("execution plan is empty".into()));
        }

        self.validate_plan_order(plan)?;
        let context = context.unwrap_or_else(|| ExecutorContext::new("executor"));
        let start = Instant::now();
        let mut outcomes = Vec::new();

        for step in &plan.steps {
            let mut outcome = ExecutorStepOutcome::new(step.id.clone());
            outcome.status = ExecutorStepStatus::InProgress;
            outcome.attempts = 0;

            let _ = self.event_bus.publish(AgentEvent::TaskStarted {
                task_id: step.id.clone(),
                description: step.description.clone(),
            });

            loop {
                outcome.attempts += 1;
                if let Err(e) = self.planner.mark_step_started(&step.id) {
                    error!("failed to mark step started {}: {}", step.id, e);
                }

                match self.execute_step(step, &context) {
                    Ok(output) => {
                        outcome.status = ExecutorStepStatus::Completed;
                        outcome.output = Some(output.clone());
                        if let Err(e) = self.planner.mark_step_completed(&step.id) {
                            error!("failed to mark step completed {}: {}", step.id, e);
                        }
                        outcome.memory_id = match self
                            .memory
                            .lock()
                            .map_err(|e| RavenError::LockPoisoned(e.to_string()))?
                            .add(MemoryKind::Working, &output, &[step.id.as_str()])
                        {
                            Ok(id) => {
                                let _ = self.event_bus.publish(AgentEvent::MemoryUpdated {
                                    memory_id: id.clone(),
                                    kind: MemoryKind::Working,
                                    tags: vec![step.id.clone()],
                                    text: output.clone(),
                                });
                                Some(id)
                            }
                            Err(e) => {
                                error!("failed to store memory: {}", e);
                                None
                            }
                        };
                        let _ = self.event_bus.publish(AgentEvent::TaskCompleted {
                            task_id: step.id.clone(),
                            output: output.clone(),
                            memory_id: outcome.memory_id.clone(),
                        });
                        break;
                    }
                    Err(err) => {
                        let err_msg = err.to_string();
                        error!(
                            "step {} failed attempt {}: {}",
                            step.id, outcome.attempts, err_msg
                        );
                        outcome.error = Some(err_msg.clone());
                        outcome.status = ExecutorStepStatus::Failed;

                        if let Err(e) = self.planner.mark_step_failed(&step.id, &err_msg) {
                            error!("failed to mark step failed {}: {}", step.id, e);
                        }

                        match self.planner.should_retry(&step.id) {
                            Ok(true) => {
                                if let Ok((attempt_no, backoff)) =
                                    self.planner.next_retry_backoff(&step.id)
                                {
                                    info!(
                                        "Retrying step {} attempt {} backoff {}",
                                        step.id, attempt_no, backoff
                                    );
                                } else {
                                    info!("Retrying step {} (attempt info unavailable)", step.id);
                                }
                                println!("DEBUG: step {} will retry", step.id);
                                continue;
                            }
                            Ok(false) => {
                                let _ = self.event_bus.publish(AgentEvent::TaskFailed {
                                    task_id: step.id.clone(),
                                    error: err_msg.clone(),
                                });

                                if step.priority >= 8 {
                                    error!("fatal failure on critical step {}", step.id);
                                    let mut cloned_plan = plan.clone();
                                    let _ =
                                        self.planner.replan_on_failure(&mut cloned_plan, &step.id);
                                    return Err(RavenError::Executor(format!(
                                        "fatal failure on step {}: {}",
                                        step.id, err_msg
                                    )));
                                }

                                outcome.status = ExecutorStepStatus::Skipped;
                                outcome.output = Some(format!("[error:{}]", err_msg));
                                break;
                            }
                            Err(e) => {
                                error!("planner error while checking retry for {}: {}", step.id, e);
                                let _ = self.event_bus.publish(AgentEvent::TaskFailed {
                                    task_id: step.id.clone(),
                                    error: e.to_string(),
                                });
                                return Err(e);
                            }
                        }
                    }
                }
            }

            outcomes.push(outcome);
        }

        if let Err(e) = self
            .memory
            .lock()
            .map_err(|e| RavenError::LockPoisoned(e.to_string()))?
            .consolidate()
        {
            error!("memory consolidation failed: {}", e);
        }

        let report = ExecutorReport::new(outcomes, start.elapsed());
        Ok(report)
    }

    fn execute_step(&self, step: &Step, context: &ExecutorContext) -> RavenResult<String> {
        if step.needs_tool {
            let tool_name = step.tool_name.as_deref().unwrap_or("echo");
            let params = step.params.as_ref().unwrap_or(&Value::Null).clone();
            let tool_context = context.clone().into_tool_context();

            let _ = self.event_bus.publish(AgentEvent::ToolCalled {
                tool_name: tool_name.to_string(),
                params: params.clone(),
            });

            let result = self
                .tools
                .lock()
                .map_err(|e| RavenError::LockPoisoned(e.to_string()))?
                .invoke(tool_name, &params, &tool_context)
                .map_err(RavenError::from);

            match result {
                Ok(tool_result) => {
                    let _ = self.event_bus.publish(AgentEvent::ToolCompleted {
                        tool_name: tool_name.to_string(),
                        result: tool_result.data.clone(),
                    });
                    Ok(format!("tool_result: {}", tool_result.data))
                }
                Err(e) => Err(e),
            }
        } else {
            self.llm.generate(&step.description, step.params.as_ref())
        }
    }

    fn validate_plan_order(&self, plan: &ExecutionPlan) -> RavenResult<()> {
        for (index, step) in plan.steps.iter().enumerate() {
            for dependency in &step.depends_on {
                let dep_index = plan.find_step_index(dependency).ok_or_else(|| {
                    RavenError::Executor(format!(
                        "step {} depends on unknown step {}",
                        step.id, dependency
                    ))
                })?;
                if dep_index >= index {
                    return Err(RavenError::Executor(format!(
                        "step {} depends on {} but plan order is invalid",
                        step.id, dependency
                    )));
                }
            }
        }
        Ok(())
    }
}
