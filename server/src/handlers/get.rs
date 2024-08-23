use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_get_http() -> &'static str {
    "Hello, world!"
}

pub async fn check_in() -> impl IntoResponse {
    StatusCode::OK
}
