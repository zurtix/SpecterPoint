use common::error::Result;
use common::models::user::AuthSession;

pub async fn logout(mut auth_session: AuthSession) -> Result<()> {
    let _ = auth_session
        .logout()
        .await
        .map_err(|_| common::error::Error::Auth);

    Ok(())
}
