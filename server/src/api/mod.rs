use crate::app::App;
use axum::routing::Router;

pub mod auth;
pub mod listeners;

pub fn routes(state: App) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new().nest("/listeners", listeners::routes()),
        )
        .with_state(state)
}
