use crate::app::App;
use axum::{extract::State, response::IntoResponse, Json};
use common::{error::Result, models::task::Task};
use comms::debug;

pub async fn add_task(
    State(state): State<App>,
    Json(task): Json<Task>,
) -> Result<impl IntoResponse> {
    debug!("Adding new task for agent [{}]", task.agent_id);
    Ok(Json(""))
}
