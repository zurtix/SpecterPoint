mod api;
mod app;
mod auth;
mod handlers;
mod listeners;
mod models;
mod orchestrator;

use crate::app::App;
use common::db::sqlite;
use tokio::sync::broadcast;

const DB_URL: &str = "sqlite://specterpoint-server.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    sqlite::init(DB_URL, Some("./migrations")).await;
    let pool = sqlite::connect(DB_URL).await;
    eventlogs::communication::COMMS.start(pool.clone()).await;
    App::new(pool).serve().await
}
