use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, RwLock};

use crate::manager::TcpManager;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub manager: TcpManager,
    pub key: Arc<RwLock<String>>,
}

impl AppState {
    pub fn new(pool: SqlitePool, manager: TcpManager) -> Self {
        Self {
            pool,
            manager,
            key: Arc::new(RwLock::new("specterpoint".to_string())),
        }
    }

    pub fn set_key(&self, key: String) {
        let mut k = self.key.write().unwrap();
        *k = key
    }
}
