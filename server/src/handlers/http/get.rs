use axum::{http::StatusCode, Extension};
use eventlogs::{agent, debug, models::agent::Agent};

pub async fn check_in(Extension(ag): Extension<String>) -> StatusCode {
    debug!("{}", ag);
    match serde_json::from_str::<Agent>(&ag) {
        Ok(agent) => {
            agent!(agent);
            StatusCode::OK
        }
        Err(_) => StatusCode::NOT_FOUND,
    }
}
