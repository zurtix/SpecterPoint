use axum::Json;
use common::error::Result;
use common::models::user::{AuthSession, Credentials};
use tracing::{error, info};

pub async fn login(mut auth_session: AuthSession, Json(creds): Json<Credentials>) -> Result<()> {
    info!("Login attempt for user [{}]", creds.username);
    if let Ok(Some(user)) = auth_session.authenticate(creds.clone()).await {
        auth_session
            .login(&user)
            .await
            .map_err(|_| common::error::Error::Auth)?;
    } else {
        error!("Failed to authenticate user: {}", creds.username);
    }

    info!("User [{}] is now logged in", creds.username);
    Ok(())
}
