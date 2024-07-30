use crate::error::Result;
use crate::models::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use tracing::debug;

pub async fn delete_listener(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    debug!("Deleting listener.");
    crate::db::listener::delete_listener(state.pool, &id).await;
    Ok(Json(""))
}
