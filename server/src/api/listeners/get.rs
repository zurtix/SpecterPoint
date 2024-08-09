use axum::{response::IntoResponse, Json};
use common::error::Result;
use tracing::debug;

pub async fn get_listeners() -> Result<impl IntoResponse> {
    debug!("Geeting listeners.");
    Ok(Json(""))
}
