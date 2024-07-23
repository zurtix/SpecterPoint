use crate::orchestrator::Orchestrator;

#[derive(Clone)]
pub struct AppState {
    pub orch: Orchestrator,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            orch: Orchestrator::new(),
        }
    }
}
