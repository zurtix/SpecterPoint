use axum::http::StatusCode;
use eventlogs::info;

pub async fn check_in() -> StatusCode {
    info!("Agent checking in!");
    StatusCode::OK
}
