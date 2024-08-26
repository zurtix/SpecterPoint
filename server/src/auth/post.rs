use axum::Json;
use common::error::Result;
use common::models::user::{AuthSession, Credentials};
use tracing::{debug, error};

pub async fn login(mut auth_session: AuthSession, Json(creds): Json<Credentials>) -> Result<()> {
    debug!("Login attempt for user: {}", creds.username);
    if let Ok(Some(user)) = auth_session.authenticate(creds.clone()).await {
        auth_session
            .login(&user)
            .await
            .map_err(|_| common::error::Error::Auth)?;
    } else {
        error!("Failed to authenticate user: {}", creds.username);
    }

    Ok(())
}
