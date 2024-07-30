mod api;
mod db;
mod error;
mod handlers;
mod models;
mod orchestrator;

use common::db::sqlite;
use models::{config::Config, state::AppState};
use tracing::info;

const DB_URL: &str = "sqlite://specterpoint-server.db";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    sqlite::init(DB_URL, Some("./migrations")).await;
    let pool = sqlite::connect(DB_URL).await;

    let config = envy::from_env::<Config>().expect("Failed to read environment variables");
    let host = format!("{}:{}", &config.host, &config.port);

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .expect("Unable to bind to host");

    let app = api::app(AppState::new(pool));

    info!("Team server started");
    axum::serve(listener, app).await.unwrap();
}
