use super::dispatcher::Dispatcher;
use super::engine::AgentRuntimeService;
use super::events::RuntimeEvents;
use super::recovery::RecoveryManager;
use super::retry::RetryManager;
use super::scheduler::Scheduler;
use super::traits::{WorkflowFactory, WorkflowFactoryImpl};
use crate::event::EventBus;
use crate::executor::Executor;
use crate::llm::Llm;
use crate::memory::MemoryStorage;
use crate::planner::PlannerProgress;
use crate::reflection::ReflectionEvaluator;
use crate::tool::ToolManagerService;
use std::sync::{Arc, Mutex};

/// Builder for AgentRuntimeService wiring dependencies. Builder is the only
/// place where concrete dependencies are wired and registered.
pub struct AgentRuntimeBuilder {
    event_bus: Option<Arc<EventBus>>,
    planner: Option<Arc<dyn PlannerProgress + Send + Sync>>,
    memory: Option<Arc<Mutex<Box<dyn MemoryStorage>>>>,
    workflow_factory: Option<Arc<dyn WorkflowFactory>>,
    tools: Option<Arc<Mutex<Box<dyn ToolManagerService>>>>,
    llm: Option<Arc<dyn Llm + Send + Sync>>,
    reflection: Option<Arc<dyn ReflectionEvaluator>>,
    dispatcher: Dispatcher,
    retry: Option<Arc<RetryManager>>,
    recovery: Option<Arc<RecoveryManager>>,
    scheduler: Option<Arc<Scheduler>>,
}

impl AgentRuntimeBuilder {
    pub fn new() -> Self {
        Self {
            event_bus: None,
            planner: None,
            memory: None,
            workflow_factory: None,
            tools: None,
            llm: None,
            reflection: None,
            dispatcher: Dispatcher::new(),
            retry: None,
            recovery: None,
            scheduler: None,
        }
    }

    pub fn with_event_bus(mut self, bus: Arc<EventBus>) -> Self {
        self.event_bus = Some(bus);
        self
    }

    pub fn with_planner(mut self, planner: Arc<dyn PlannerProgress + Send + Sync>) -> Self {
        self.planner = Some(planner);
        self
    }

    pub fn with_memory(mut self, memory: Arc<Mutex<Box<dyn MemoryStorage>>>) -> Self {
        self.memory = Some(memory);
        self
    }

    pub fn with_tools(mut self, tools: Arc<Mutex<Box<dyn ToolManagerService>>>) -> Self {
        self.tools = Some(tools.clone());
        self.dispatcher = self.dispatcher.with_tools(tools);
        self
    }

    pub fn with_llm(mut self, llm: Arc<dyn Llm + Send + Sync>) -> Self {
        self.llm = Some(llm);
        self
    }

    pub fn with_reflection(mut self, reflection: Arc<dyn ReflectionEvaluator>) -> Self {
        self.reflection = Some(reflection);
        self
    }

    pub fn register_executor(self, name: impl Into<String>, exe: Arc<dyn Executor>) -> Self {
        self.dispatcher.register_executor(name, exe);
        self
    }

    pub fn with_workflow_factory(mut self, factory: Arc<dyn WorkflowFactory>) -> Self {
        self.workflow_factory = Some(factory);
        self
    }

    pub fn with_retry_manager(mut self, retry: Arc<RetryManager>) -> Self {
        self.retry = Some(retry);
        self
    }

    pub fn with_recovery_manager(mut self, rec: Arc<RecoveryManager>) -> Self {
        self.recovery = Some(rec);
        self
    }

    pub fn with_scheduler(mut self, scheduler: Arc<Scheduler>) -> Self {
        self.scheduler = Some(scheduler);
        self
    }

    /// Build AgentRuntimeService; returns Err if required dependencies missing.
    pub fn build(self) -> Result<AgentRuntimeService, String> {
        let bus = self
            .event_bus
            .ok_or_else(|| "event_bus is required".to_string())?;
        let planner = self
            .planner
            .ok_or_else(|| "planner is required".to_string())?;
        let memory = self
            .memory
            .ok_or_else(|| "memory is required".to_string())?;
        let tools = self
            .tools
            .ok_or_else(|| "tools manager is required".to_string())?;
        let llm = self.llm.ok_or_else(|| "llm is required".to_string())?;
        let reflection = self
            .reflection
            .ok_or_else(|| "reflection is required".to_string())?;
        let retry = self.retry.unwrap_or_else(|| Arc::new(RetryManager::new(3)));
        let recovery = self
            .recovery
            .unwrap_or_else(|| Arc::new(RecoveryManager::new()));
        let scheduler = self.scheduler.unwrap_or_else(|| Arc::new(Scheduler::new()));

        let events = RuntimeEvents::new(bus.clone());
        let mut dispatcher = self.dispatcher;
        dispatcher = dispatcher.with_event_bus(bus.clone());

        let workflow_factory = self.workflow_factory.unwrap_or_else(|| {
            Arc::new(WorkflowFactoryImpl::new(
                planner.clone(),
                memory.clone(),
                tools.clone(),
                llm.clone(),
                reflection.clone(),
                bus.clone(),
            )) as Arc<dyn WorkflowFactory>
        });

        Ok(AgentRuntimeService::new(
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
        ))
    }
}

impl Default for AgentRuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
