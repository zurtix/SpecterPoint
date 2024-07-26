use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        let filename = "specterpoint-client.db";
        let db_url = format!("sqlite://{}", filename);

        if !sqlx::Sqlite::database_exists(&db_url).await.unwrap() {
            sqlx::Sqlite::create_database(&db_url).await.unwrap();
        }

        let pool = SqlitePool::connect(&db_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("../migrations").run(&pool).await.unwrap();

        Self { pool }
    }
}
