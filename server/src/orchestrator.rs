use crate::db::listener::get_listener;
use crate::models::listener::Listen;
use crate::models::listener::{HttpListener, HttpsListener, TcpListener};
use axum::extract::FromRef;
use common::models::listener::ListenerTypes;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
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

    pub async fn start_listener(self, id: &i64) {
        let lstn = get_listener(self.pool, id).await;
        let listener: Box<dyn Listen + Send> = match lstn.listener.r#type {
            ListenerTypes::Http => Box::new(
                HttpListener::new(lstn.listener.host, lstn.listener.port, lstn.endpoints).await,
            ) as Box<dyn Listen + Send>,
            ListenerTypes::Https => Box::new(HttpsListener::new()) as Box<dyn Listen + Send>,
            ListenerTypes::Tcp => Box::new(TcpListener::new()) as Box<dyn Listen + Send>,
        };
        let mut running = self.running.lock().await;

        running.entry(lstn.listener.id).or_insert(listener);

        if let Some(listener) = running.get_mut(id) {
            listener.start();
        }
    }

    pub async fn stop_listener(&self, id: &i64) {
        let mut listeners = self.running.lock().await;
        if let Some(listener) = listeners.get_mut(id) {
            listener.stop();
        }
    }

    pub async fn is_running(&self, id: &i64) -> bool {
        (*(self.running.lock().await)).contains_key(id)
    }
}
