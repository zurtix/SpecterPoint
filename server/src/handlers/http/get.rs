use axum::{http::StatusCode, Extension};
use eventlogs::{checkin, debug, models::agent::Agent};

pub async fn check_in(Extension(id): Extension<String>) -> StatusCode {
    debug!("Agent id: {}", id);
    checkin!(Agent {
        id,
        last_seen: chrono::Utc::now().to_rfc3339()
    });
    StatusCode::OK
}
