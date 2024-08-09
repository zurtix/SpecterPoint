mod get;
mod post;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/logout", get(self::get::logout))
}
