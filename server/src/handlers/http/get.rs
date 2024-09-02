use axum::http::StatusCode;
use eventlogs::{agent, models::agent::Agent};

pub async fn check_in() -> StatusCode {
    let id = "test".to_string();
    let agent = Agent { id };
    agent!(agent);

    StatusCode::OK
}
