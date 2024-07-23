mod api;
mod error;
mod handlers;
mod models;
mod orchestrator;

use models::{config::Config, state::AppState};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = envy::from_env::<Config>().expect("Failed to read environment variables");
    let host = format!("{}:{}", &config.host, &config.port);

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .expect("Unable to bind to host");

    let app = api::app(AppState::new());

    info!("Team server started");
    axum::serve(listener, app).await.unwrap();
}
