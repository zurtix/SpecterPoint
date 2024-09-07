use crate::listeners::Listen;
use crate::listeners::{http::HttpListener, https::HttpsListener, tcp::TcpListener};
use axum::extract::FromRef;
use common::models::listener::ListenerFull;
use common::{error::Result, models::listener::ListenerTypes};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::RsaPrivateKey;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::tasks::TaskManager;

type Listeners = HashMap<i64, Box<dyn Listen + Send>>;

#[derive(Clone, FromRef, Default)]
pub struct ListenerManager {
    listeners: Arc<Mutex<Listeners>>,
}

impl ListenerManager {
    pub async fn start(self, lstn: ListenerFull, task_manager: TaskManager) -> Result<()> {
        let listener: Box<dyn Listen + Send> = match lstn.inner.listener.r#type {
            ListenerTypes::Http => Box::new(
                HttpListener::new(
                    lstn.to_string(),
                    lstn.endpoints,
                    lstn.metadata,
                    RsaPrivateKey::from_pkcs1_pem(&lstn.inner.listener.private_key)?,
                    task_manager,
                )
                .await,
            ) as Box<dyn Listen + Send>,
            ListenerTypes::Https => Box::new(HttpsListener::new()) as Box<dyn Listen + Send>,
            ListenerTypes::Tcp => Box::new(TcpListener::new()) as Box<dyn Listen + Send>,
        };
        let mut running = self.listeners.lock().await;

        running.entry(lstn.inner.id).or_insert(listener);

        if let Some(listener) = running.get_mut(&lstn.inner.id) {
            listener.start().await;
        }

        Ok(())
    }

    pub async fn stop(&self, id: &i64) {
        let mut listeners = self.listeners.lock().await;
        if let Some(listener) = listeners.get_mut(id) {
            listener.stop().await;
        }
    }

    pub async fn status(&self, id: &i64) -> bool {
        (*(self.listeners.lock().await)).contains_key(id)
    }
}
