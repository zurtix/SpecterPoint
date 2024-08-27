use crate::app::App;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::crypt::hash::{self, generate_password_hash};
use common::{error::Result, models::user::Credentials};
use tracing::debug;

pub async fn add_user(
    State(state): State<App>,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse> {
    debug!("Adding new user {}", creds.auth.username);
    let pass = generate_password_hash(&creds.auth.password)?;
    common::db::user::create_user(state.pool, creds.auth).await?;
    Ok(Json(""))
}
