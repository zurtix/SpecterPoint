use crate::app::App;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::error::Result;
use tracing::debug;

pub async fn delete_listener(
    State(state): State<App>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    debug!("Deleting listener.");
    crate::db::listener::delete_listener(state.pool, &id).await;
    Ok(Json(""))
}
