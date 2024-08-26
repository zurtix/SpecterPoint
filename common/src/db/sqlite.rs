use std::{path::Path, str::FromStr};

use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::{SqliteConnectOptions, SqlitePool},
    ConnectOptions,
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
    let opts = SqliteConnectOptions::from_str(db_url)
        .expect("Failed to create connection options")
        .disable_statement_logging();
    SqlitePool::connect_with(opts)
        .await
        .expect("Failed to connect to database")
}
