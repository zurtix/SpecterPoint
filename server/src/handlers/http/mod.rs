use axum::routing::{self, Router};

use crate::managers::tasks::TaskManager;
pub mod get;
pub mod post;

pub fn routes(route: &str) -> Router<TaskManager> {
    Router::new().route(route, routing::get(get::check_in).post(post::receive))
}
