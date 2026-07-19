use crate::event::{AgentEvent, EventBus};
use std::sync::Arc;

pub struct RuntimeEvents {
    bus: Arc<EventBus>,
}

impl RuntimeEvents {
    pub fn new(bus: Arc<EventBus>) -> Self {
        Self { bus }
    }

    pub fn publish(&self, ev: AgentEvent) {
        let _ = self.bus.publish(ev);
    }

    /// Expose inner EventBus for components that require direct access.
    pub fn bus(&self) -> Arc<EventBus> {
        self.bus.clone()
    }
}
