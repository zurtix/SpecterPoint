use crate::error::Result;
use crate::models::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::models::listener::CreateListener;
use tracing::debug;

pub async fn create_listener(
    State(state): State<AppState>,
    Json(create_listener): Json<CreateListener>,
) -> Result<impl IntoResponse> {
    debug!("Creating new listener.");
    state.orch.add_listener(create_listener).await;
    Ok(Json(""))
}

pub async fn start_listener(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse> {
    debug!("Starting {}", name);
    state.orch.start_listener(&name).await;
    Ok(Json(""))
}

pub async fn stop_listener(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse> {
    debug!("Stopping {}", name);
    state.orch.stop_listener(&name).await;
    Ok(Json(""))
}
