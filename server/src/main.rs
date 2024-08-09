mod api;
mod app;
mod auth;
mod db;
mod handlers;
mod models;
mod orchestrator;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    App::new().await?.serve().await
}
