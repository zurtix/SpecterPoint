use axum::{http::StatusCode, Extension};
use comms::{checkin, debug, models::agent::Agent};

pub async fn check_in(Extension(id): Extension<String>) -> StatusCode {
    debug!("Agent checking in [{}]", id);
    checkin!(Agent::new(id));

    //TODO: Return a list of tasks for the agent to execute

    StatusCode::OK
}
