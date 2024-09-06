use crate::orchestrator::Orchestrator;
use crate::{api, auth, models::config::Config};
use axum_login::{
    login_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use common::models::user::Backend;
use comms::info;
use sqlx::SqlitePool;
use tower_sessions::cookie::{time::Duration, Key};
use tower_sessions_sqlx_store::SqliteStore;

#[derive(Clone)]
pub struct App {
    pub orch: Orchestrator,
    pub pool: SqlitePool,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            orch: Orchestrator::new(pool.clone()),
            pool,
        }
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = envy::from_env::<Config>().expect("Failed to read environment variables");

        let session_store = SqliteStore::new(self.pool.clone());
        session_store
            .migrate()
            .await
            .expect("Failed to migrate session store");

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::seconds(10)))
            .with_signed(key);

        let backend = Backend::new(self.pool.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let host = format!("{}:{}", &config.host, &config.port);

        let app = api::routes(self)
            .route_layer(login_required!(Backend))
            .merge(auth::router())
            .layer(auth_layer);

        let listener = tokio::net::TcpListener::bind(host)
            .await
            .expect("Unable to bind to host");

        info!("Server started");
        axum::serve(listener, app).await?;
        deletion_task.await??;

        Ok(())
    }
}
