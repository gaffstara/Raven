use super::dispatcher::Dispatcher;
use super::events::RuntimeEvents;
use super::recovery::RecoveryManager;
use super::report::RuntimeReport;
use super::retry::RetryManager;
use super::scheduler::Scheduler;
use super::traits::WorkflowFactory;
use crate::error::{RavenError, RavenResult};
use crate::llm::Llm;
use crate::memory::{MemoryKind, MemoryStorage};
use crate::planner::ExecutionPlan;
use crate::planner::PlannerProgress;
use crate::reflection::ReflectionEvaluator;
use crate::tool::ToolManagerService;
use log::{error, info};
use std::sync::{Arc, Mutex};

/// AgentRuntimeService: orchestrates subsystems to run a plan end-to-end.
pub struct AgentRuntimeService {
    events: RuntimeEvents,
    dispatcher: Dispatcher,
    planner: Arc<dyn PlannerProgress + Send + Sync>,
    memory: Arc<Mutex<Box<dyn MemoryStorage>>>,
    tools: Arc<Mutex<Box<dyn ToolManagerService>>>,
    llm: Arc<dyn Llm + Send + Sync>,
    reflection: Arc<dyn ReflectionEvaluator>,
    workflow_factory: Arc<dyn WorkflowFactory>,
    retry: Arc<RetryManager>,
    recovery: Arc<RecoveryManager>,
    scheduler: Arc<Scheduler>,
}

impl AgentRuntimeService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        events: RuntimeEvents,
        dispatcher: Dispatcher,
        planner: Arc<dyn PlannerProgress + Send + Sync>,
        memory: Arc<Mutex<Box<dyn MemoryStorage>>>,
        tools: Arc<Mutex<Box<dyn ToolManagerService>>>,
        llm: Arc<dyn Llm + Send + Sync>,
        reflection: Arc<dyn ReflectionEvaluator>,
        workflow_factory: Arc<dyn WorkflowFactory>,
        retry: Arc<RetryManager>,
        recovery: Arc<RecoveryManager>,
        scheduler: Arc<Scheduler>,
    ) -> Self {
        Self {
            events,
            dispatcher,
            planner,
            memory,
            tools,
            llm,
            reflection,
            workflow_factory,
            retry,
            recovery,
            scheduler,
        }
    }

    /// Run an execution plan end-to-end and return a summary string report.
    pub fn run_plan(&self, plan: &ExecutionPlan) -> RavenResult<String> {
        info!("AgentRuntime: starting plan {}", plan.steps.len());
        let mut report =
            RuntimeReport::new(crate::agent::runtime::state::LifecycleState::Execution);

        let workflow_id = plan
            .steps
            .first()
            .map(|s| format!("workflow-{}", s.id))
            .unwrap_or_else(|| "workflow-unknown".to_string());

        self.events
            .publish(crate::event::AgentEvent::WorkflowStarted {
                workflow_id: workflow_id.clone(),
            });

        // Use memory to retrieve contextual items related to the plan's first step
        if let Ok(mem_guard) = self.memory.lock() {
            if let Some(first) = plan.steps.first() {
                let query = &first.description;
                let _ctx = mem_guard.retrieve(query, None, 5);
            }
        }

        // Use LLM to produce a small pre-run context (non-fatal)
        if let Some(first) = plan.steps.first() {
            let prompt = format!("Prepare execution for: {}", first.description);
            let _ = self.llm.generate(&prompt, None);
        }

        // Validate and schedule plan
        let scheduled = self
            .scheduler
            .schedule(plan)
            .map_err(|e| RavenError::Planner(format!("scheduler error: {}", e)))?;

        // Load the scheduled plan into progress tracking before execution.
        self.planner
            .load_plan(&scheduled)
            .map_err(|e| RavenError::Planner(format!("planner load error: {}", e)))?;

        let mut attempt = 0;
        loop {
            attempt += 1;

            // Select executor via dispatcher
            let executor = self
                .dispatcher
                .select_executor(&scheduled)
                .ok_or_else(|| RavenError::Configuration("no suitable executor found".into()))?;

            // Optionally inspect tools registry for the first step
            if let Some(first) = scheduled.steps.first() {
                if let Some(tool_name) = &first.tool_name {
                    if let Ok(tools_guard) = self.tools.lock() {
                        let _has = tools_guard.has_tool(tool_name);
                        let _ = _has; // keep value used to avoid warnings
                    }
                }
            }

            // Build workflow controller via injected factory.
            let workflow_controller = self.workflow_factory.build(executor.clone());
            let res = workflow_controller.start(scheduled.clone());
            match res {
                Ok(s) => {
                    report.complete();

                    // After successful run, call reflection.evaluate and commit results to memory
                    let status = workflow_controller.status();
                    let reflection_report = self.reflection.evaluate(&workflow_id, &status);
                    let summary = reflection_report.summarize();
                    if let Ok(mem_guard) = self.memory.lock() {
                        let _ = self
                            .reflection
                            .commit(mem_guard.as_ref(), reflection_report);
                        // also persist final string summary into episodic memory
                        let _ = mem_guard.add(
                            MemoryKind::Episodic,
                            &summary,
                            &["workflow", "reflection"],
                        );
                    }

                    self.events
                        .publish(crate::event::AgentEvent::WorkflowFinished {
                            workflow_id: workflow_id.clone(),
                            result: Ok(s.clone()),
                        });
                    return Ok(s);
                }
                Err(e) => {
                    error!("AgentRuntime plan failed attempt {}: {}", attempt, e);
                    if self.retry.should_retry(attempt) {
                        self.recovery.recover(&e.to_string());
                        info!("Retrying workflow plan, attempt {}", attempt + 1);
                        continue;
                    }

                    // On final failure, still evaluate and commit reflection to memory
                    let status = workflow_controller.status();
                    let reflection_report = self.reflection.evaluate(&workflow_id, &status);
                    if let Ok(mem_guard) = self.memory.lock() {
                        let _ = self
                            .reflection
                            .commit(mem_guard.as_ref(), reflection_report);
                    }

                    self.events
                        .publish(crate::event::AgentEvent::WorkflowFinished {
                            workflow_id: workflow_id.clone(),
                            result: Err(e.to_string()),
                        });
                    return Err(e);
                }
            }
        }
    }

    /// Access the runtime dispatcher for executor selection and inspection.
    pub fn dispatcher(&self) -> &Dispatcher {
        &self.dispatcher
    }
}
