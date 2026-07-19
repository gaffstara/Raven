use crate::agent::runtime::builder::AgentRuntimeBuilder;
use crate::planner::PlannerService;
use std::sync::Arc;

#[test]
fn runtime_runs_plan_via_workflow() {
    // wire minimal runtime using existing services via builder/registry
    let bus = Arc::new(crate::event::EventBus::new());
    let tools = Arc::new(std::sync::Mutex::new(
        Box::new(crate::tool::ToolService::new()) as Box<dyn crate::tool::ToolManagerService>,
    ));
    // register echo tool
    if let Ok(g) = tools.lock() {
        let _ = g.register_tool(Box::new(crate::tool::tools::EchoTool::new()));
    }

    let planner = Arc::new(PlannerService::new("rt-planner"));
    let memory = Arc::new(std::sync::Mutex::new(
        Box::new(crate::memory::MemoryService::new()) as Box<dyn crate::memory::MemoryStorage>,
    ));
    let llm = Arc::new(crate::llm::SimpleLlm::new());
    let reflection = Arc::new(crate::reflection::ReflectionService::new());

    let executor = Arc::new(crate::executor::ExecutorService::new(
        memory.clone(),
        tools.clone(),
        llm.clone(),
        planner.clone(),
        bus.clone(),
    ));

    let builder = AgentRuntimeBuilder::new()
        .with_event_bus(bus.clone())
        .with_planner(planner.clone())
        .with_memory(memory.clone())
        .with_tools(tools.clone())
        .with_llm(llm.clone())
        .with_reflection(reflection.clone())
        .register_executor("default", executor.clone());

    let runtime = builder.build().expect("builder succeeded");

    // create a plan using planner service
    let plan = planner
        .create_plan(&crate::intent::Intent {
            name: "test".to_string(),
            confidence: 1.0,
            requires_tool: false,
            requires_planner: true,
            metadata: Default::default(),
            raw: "Say hello".to_string(),
        })
        .expect("plan created");

    let res = runtime.run_plan(&plan);
    assert!(res.is_ok());
    let out = res.unwrap();
    assert!(out.contains("LLM response") || out.contains("tool_result"));
}
