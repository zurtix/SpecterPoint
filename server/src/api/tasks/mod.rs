use axum::{
    routing::{get, post},
    Router,
};

use crate::app::App;

pub mod get;
pub mod post;

pub fn routes() -> Router<App> {
    Router::new()
        .route("/:id", get(get::tasks).post(post::add_task))
        .route("/:agent_id/:task_id", post(post::mark_complete))
}
