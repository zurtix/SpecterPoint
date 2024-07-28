use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
