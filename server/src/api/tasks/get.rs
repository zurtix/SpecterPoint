use crate::app::App;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::error::Result;

pub async fn tasks(
    State(state): State<App>,
    Path(agent_id): Path<String>,
) -> Result<impl IntoResponse> {
    let tasks = state.task_manager.tasks(agent_id).await;
    Ok(Json(tasks))
}
