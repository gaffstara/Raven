use crate::error::{RavenError, RavenResult};
use crate::planner::{ExecutionPlan, Step};
use std::collections::{BinaryHeap, HashMap};

/// Scheduler performs DAG validation and produces a deterministic execution order.
/// It honors step priority (higher `priority` value means higher priority).
pub struct Scheduler {
    pub concurrency_limit: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            concurrency_limit: 1,
        }
    }
    pub fn with_concurrency(limit: usize) -> Self {
        Self {
            concurrency_limit: limit,
        }
    }

    /// Schedule returns a new ExecutionPlan ordered according to dependencies and priority.
    pub fn schedule(&self, plan: &ExecutionPlan) -> RavenResult<ExecutionPlan> {
        // Validate non-empty
        if plan.steps.is_empty() {
            return Err(RavenError::Planner("empty plan".into()));
        }

        // Build graph
        let mut indeg: HashMap<String, usize> = HashMap::new();
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        let mut nodes: HashMap<String, Step> = HashMap::new();

        for step in &plan.steps {
            indeg.entry(step.id.clone()).or_insert(0);
            nodes.insert(step.id.clone(), step.clone());
        }

        for step in &plan.steps {
            for dep in &step.depends_on {
                if !nodes.contains_key(dep) {
                    return Err(RavenError::Planner(format!(
                        "missing dependency {} for step {}",
                        dep, step.id
                    )));
                }
                *indeg.entry(step.id.clone()).or_insert(0) += 1;
                adj.entry(dep.clone())
                    .or_default()
                    .push(step.id.clone());
            }
        }

        // Use Kahn's algorithm with priority selection for nodes with indegree 0
        // BinaryHeap to pick highest priority next
        #[derive(Eq, PartialEq)]
        struct QItem {
            priority: u8,
            id: String,
            estimated_cost: u32,
        }
        impl Ord for QItem {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                // higher priority first; if equal, lower estimated_cost first
                match self.priority.cmp(&other.priority) {
                    std::cmp::Ordering::Equal => other.estimated_cost.cmp(&self.estimated_cost),
                    other => other,
                }
            }
        }
        impl PartialOrd for QItem {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap = BinaryHeap::new();
        for (id, &deg) in indeg.iter() {
            if deg == 0 {
                if let Some(s) = nodes.get(id) {
                    heap.push(QItem {
                        priority: s.priority,
                        id: id.clone(),
                        estimated_cost: s.estimated_cost,
                    });
                }
            }
        }

        let mut ordered: Vec<Step> = Vec::new();
        let mut processed = 0usize;
        while let Some(item) = heap.pop() {
            let id = item.id;
            if let Some(step) = nodes.get(&id) {
                ordered.push(step.clone());
            }
            processed += 1;
            if let Some(neighbors) = adj.get(&id) {
                for n in neighbors {
                    if let Some(v) = indeg.get_mut(n) {
                        *v -= 1;
                        if *v == 0 {
                            if let Some(s) = nodes.get(n) {
                                heap.push(QItem {
                                    priority: s.priority,
                                    id: n.clone(),
                                    estimated_cost: s.estimated_cost,
                                });
                            }
                        }
                    }
                }
            }
        }

        if processed != nodes.len() {
            return Err(RavenError::Planner(
                "cycle detected in plan dependencies".into(),
            ));
        }

        Ok(ExecutionPlan { steps: ordered })
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
