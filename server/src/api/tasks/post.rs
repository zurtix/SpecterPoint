use crate::app::App;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::{error::Result, models::task::Task};
use comms::{debug, trace};

pub async fn add_task(
    State(state): State<App>,
    Path(agent_id): Path<String>,
    Json(task): Json<Task>,
) -> Result<impl IntoResponse> {
    debug!("Adding new task for agent [{}]", task.agent_id);
    state.task_manager.add(agent_id, task).await;
    Ok(Json(""))
}

pub async fn mark_complete(
    State(state): State<App>,
    Path((agent_id, task_idx)): Path<(String, usize)>,
) -> Result<impl IntoResponse> {
    trace!(
        "Marking task [{}] for agent [{}] as complete",
        task_idx,
        agent_id
    );
    state.task_manager.complete(agent_id, task_idx).await;

    Ok(Json(""))
}
