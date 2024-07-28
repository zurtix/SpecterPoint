use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool};

static DB_URL: &str = "sqlite://specterpoint-client.db";

pub async fn init() {
    if !sqlx::Sqlite::database_exists(DB_URL).await.unwrap() {
        sqlx::Sqlite::create_database(DB_URL).await.unwrap();
        sqlx::migrate!("./migrations")
            .run(&connect().await)
            .await
            .unwrap();
    }
}

pub async fn connect() -> SqlitePool {
    SqlitePool::connect(DB_URL)
        .await
        .expect("Failed to connect to database")
}
