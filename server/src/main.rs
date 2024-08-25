mod api;
mod app;
mod auth;
mod handlers;
mod models;
mod orchestrator;
mod subscriber;

use models::config::Config;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = envy::from_env::<Config>().expect("Failed to read environment variables");
    subscriber::init(&config.host, config.log_port).await;
    App::new().await?.serve(config).await
}
