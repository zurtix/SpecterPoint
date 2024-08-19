use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub key: Arc<RwLock<String>>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            key: Arc::new(RwLock::new("specterpoint".to_string())),
        }
    }

    pub fn set_key(&self, key: String) {
        let mut k = self.key.write().unwrap();
        *k = key
    }
}
