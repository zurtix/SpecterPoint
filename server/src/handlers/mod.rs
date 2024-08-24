use axum::{routing::get, Router};
pub mod agent;

pub fn routes(route: &str) -> Router {
    Router::new().route(route, get(agent::get::check_in).post(agent::post::receive))
}
