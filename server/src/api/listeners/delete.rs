use crate::error::Result;
use crate::models::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use tracing::debug;

pub async fn delete_listener(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse> {
    debug!("Deleting listener.");
    state.orch.remove_listener(&name).await;
    Ok(Json(""))
}
