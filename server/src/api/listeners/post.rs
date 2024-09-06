use crate::app::App;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::error::Result;
use common::models::listener::ListenerFull;
use comms::info;

pub async fn add_listener(
    State(state): State<App>,
    Json(listener): Json<ListenerFull>,
) -> Result<impl IntoResponse> {
    info!("Adding new listener {}", listener.listener.id);
    common::db::listener::add_listener(state.pool, listener).await?;
    Ok(Json(""))
}

pub async fn start_listener(
    State(state): State<App>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    info!("Starting listener {}", id);
    state.orch.start_listener(&id).await?;
    Ok(Json(""))
}

pub async fn stop_listener(
    State(state): State<App>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    info!("Stopping listener {}", id);
    state.orch.stop_listener(&id).await;
    Ok(Json(""))
}
