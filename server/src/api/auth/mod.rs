mod get;
mod post;

use crate::app::App;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/logout", get(self::get::logout))
}
