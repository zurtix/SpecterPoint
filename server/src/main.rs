mod api;
mod app;
mod db;
mod handlers;
mod middleware;
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
