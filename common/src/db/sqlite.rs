use std::path::Path;

use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePool,
};

pub async fn init(db_url: &str, migrations: Option<&str>) {
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap() {
        sqlx::Sqlite::create_database(db_url).await.unwrap();
        if let Some(m) = migrations {
            let migrator = Migrator::new(Path::new(m)).await.unwrap();
            let pool = connect(db_url).await;
            migrator.run(&pool).await.unwrap();
        }
    }
}

pub async fn connect(db_url: &str) -> SqlitePool {
    SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to database")
}
