use crate::listeners::Listen;
use crate::listeners::{http::HttpListener, https::HttpsListener, tcp::TcpListener};
use axum::extract::FromRef;
use common::db::listener::get_listener;
use common::{error::Result, models::listener::ListenerTypes};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::RsaPrivateKey;
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, FromRef)]
pub struct Orchestrator {
    running: Arc<Mutex<HashMap<i64, Box<dyn Listen + Send>>>>,
    pool: SqlitePool,
}

impl Orchestrator {
    pub fn new(pool: SqlitePool) -> Self {
        Orchestrator {
            running: Arc::new(Mutex::new(HashMap::new())),
            pool,
        }
    }

    pub async fn start_listener(self, id: &i64) -> Result<()> {
        let lstn = get_listener(self.pool, id).await?;

        let listener: Box<dyn Listen + Send> = match lstn.listener.listener.r#type {
            ListenerTypes::Http => Box::new(
                HttpListener::new(
                    lstn.listener.listener.host,
                    lstn.listener.listener.port,
                    RsaPrivateKey::from_pkcs1_pem(&lstn.listener.listener.private_key)?,
                    lstn.endpoints,
                    lstn.metadata,
                )
                .await,
            ) as Box<dyn Listen + Send>,
            ListenerTypes::Https => Box::new(HttpsListener::new()) as Box<dyn Listen + Send>,
            ListenerTypes::Tcp => Box::new(TcpListener::new()) as Box<dyn Listen + Send>,
        };
        let mut running = self.running.lock().await;

        running.entry(lstn.listener.id).or_insert(listener);

        if let Some(listener) = running.get_mut(id) {
            listener.start().await;
        }

        Ok(())
    }

    pub async fn stop_listener(&self, id: &i64) {
        let mut listeners = self.running.lock().await;
        if let Some(listener) = listeners.get_mut(id) {
            listener.stop().await;
        }
    }

    pub async fn is_running(&self, id: &i64) -> bool {
        (*(self.running.lock().await)).contains_key(id)
    }
}
