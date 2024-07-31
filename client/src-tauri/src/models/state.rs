use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub key: String,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            key: "specterpoint".to_string(),
        }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key
    }
}
