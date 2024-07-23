use crate::models::state::AppState;
use axum::routing::Router;

pub mod listeners;

pub fn app(state: AppState) -> Router {
    Router::new().nest("/api", routes()).with_state(state)
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/listeners", listeners::routes())
}
