use axum::routing::{self, Router};
pub mod get;
pub mod post;

pub fn routes(route: &str) -> Router {
    Router::new().route(route, routing::get(get::check_in).post(post::receive))
}
