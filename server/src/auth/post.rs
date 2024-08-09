use axum::Json;
use common::error::Result;
use common::models::user::{AuthSession, Credentials};

pub async fn login(mut auth_session: AuthSession, Json(creds): Json<Credentials>) -> Result<()> {
    if let Ok(Some(user)) = auth_session.authenticate(creds.clone()).await {
        auth_session
            .login(&user)
            .await
            .map_err(|_| common::error::Error::Auth)?;
    };

    Ok(())
}
