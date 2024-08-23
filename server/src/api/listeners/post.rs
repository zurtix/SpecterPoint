use crate::app::App;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::error::Result;
use common::models::listener::Listener;
use tracing::debug;

pub async fn add_listener(
    State(state): State<App>,
    Json(listener): Json<Listener>,
) -> Result<impl IntoResponse> {
    debug!("Adding new listener {}", listener.listener.id);
    crate::db::listener::add_listener(state.pool, listener).await?;
    Ok(Json(""))
}

pub async fn start_listener(
    State(state): State<App>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    debug!("Starting {}", id);
    state.orch.start_listener(&id).await?;
    Ok(Json(""))
}

pub async fn stop_listener(
    State(state): State<App>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    debug!("Stopping {}", id);
    state.orch.stop_listener(&id).await;
    Ok(Json(""))
}
