use comms::event::EventManager;
use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub eventlogs: EventManager,
    pub key: Arc<RwLock<String>>,
}

impl AppState {
    pub fn new(pool: SqlitePool, eventlogs: EventManager) -> Self {
        Self {
            pool,
            eventlogs,
            key: Arc::new(RwLock::new("specterpoint".to_string())),
        }
    }

    pub fn set_key(&self, key: String) {
        let mut k = self.key.write().unwrap();
        *k = key
    }
}
