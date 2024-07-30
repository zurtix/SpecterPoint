use crate::orchestrator::Orchestrator;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub orch: Orchestrator,
    pub pool: SqlitePool,
}
impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            orch: Orchestrator::new(pool.clone()),
            pool,
        }
    }
}
