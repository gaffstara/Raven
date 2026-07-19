use crate::executor::ExecutorService;
use crate::llm::SimpleLlm;
use crate::memory::MemoryService;
use crate::planner::{PlannerProgress, PlannerService};
use crate::tool::ToolService;
use std::sync::{Arc, Mutex};

#[test]
fn executor_service_runs_a_plan() {
    let planner_service = PlannerService::new("test-planner");
    let planner: Arc<dyn PlannerProgress + Send + Sync> = Arc::new(planner_service);
    let memory: Arc<Mutex<Box<dyn crate::memory::MemoryStorage>>> =
        Arc::new(Mutex::new(Box::new(MemoryService::new())));
    let tools: Arc<Mutex<Box<dyn crate::tool::ToolManagerService>>> =
        Arc::new(Mutex::new(Box::new(ToolService::new())));
    let llm = Arc::new(SimpleLlm::new());
    let event_bus = Arc::new(crate::event::EventBus::new());

    let executor = ExecutorService::new(
        Arc::clone(&memory),
        Arc::clone(&tools),
        llm,
        Arc::clone(&planner),
        Arc::clone(&event_bus),
    );

    let plan = PlannerService::new("test-planner")
        .create_plan(&crate::intent::Intent {
            name: "general".to_string(),
            confidence: 1.0,
            requires_tool: false,
            requires_planner: true,
            metadata: Default::default(),
            raw: "Say hello".to_string(),
        })
        .expect("plan creation should succeed");

    let result = executor
        .execute_plan(&plan)
        .expect("execution should succeed");

    assert!(result.contains("ExecutorReport"));
    assert!(result.contains("completed"));
}
