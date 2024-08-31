use crate::app::App;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::error::Result;
use eventlogs::debug;

pub async fn delete_listener(
    State(state): State<App>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    debug!("Deleting listener.");
    common::db::listener::delete_listener(state.pool, &id).await?;
    Ok(Json(""))
}
