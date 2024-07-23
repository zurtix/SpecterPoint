use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::models::state::AppState;

pub mod delete;
pub mod get;
pub mod post;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get::get_listeners).post(post::create_listener))
        .route("/:name/start", post(post::start_listener))
        .route("/:name/stop", post(post::stop_listener))
        .route("/:name", delete(delete::delete_listener))
}
