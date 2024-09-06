use crate::app::App;
use axum::{extract::State, response::IntoResponse, Json};
use common::crypt::hash::generate_password_hash;
use common::models::user::BaseCredential;
use common::{error::Result, models::user::Credentials};
use comms::debug;

pub async fn add_user(
    State(state): State<App>,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse> {
    debug!("Adding new user {}", creds.auth.username);
    let password = generate_password_hash(&creds.auth.password)?;
    common::db::user::create_user(
        state.pool,
        BaseCredential {
            username: creds.auth.username,
            password,
        },
    )
    .await
}
