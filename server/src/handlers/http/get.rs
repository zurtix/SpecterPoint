use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use comms::{checkin, debug, models::agent::Agent};

use crate::managers::tasks::TaskManager;

pub async fn check_in(
    State(task_manager): State<TaskManager>,
    Extension(id): Extension<String>,
) -> impl IntoResponse {
    debug!("Agent checking in [{}]", id);
    checkin!(Agent::new(id.clone()));

    let tasks = task_manager.tasks(id.clone()).await;

    if !tasks.is_empty() {
        debug!("Agent [{}] acquired {} tasks", id, tasks.len());
    }

    (StatusCode::OK, Json(tasks))
}
