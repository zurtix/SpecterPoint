use crate::error::Result;
use axum::{response::IntoResponse, Json};
use tracing::debug;

pub async fn get_listeners() -> Result<impl IntoResponse> {
    debug!("Geeting listeners.");
    Ok(Json(""))
}
