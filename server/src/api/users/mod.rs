pub mod post;

use crate::app::App;
use axum::{routing::post, Router};

pub fn routes() -> Router<App> {
    Router::new().route("/", post(post::add_user))
}
