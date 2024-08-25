mod api;
mod app;
mod auth;
mod handlers;
mod models;
mod orchestrator;
mod subscriber;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    subscriber::init().await;
    App::new().await?.serve().await
}
