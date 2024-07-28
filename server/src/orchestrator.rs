use crate::models::listener::Listener;
use crate::models::listener::{HttpListener, HttpsListener, TcpListener};
use axum::extract::FromRef;
use common::models::listener::{CreateListener, ListenerTypes};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::sync::Mutex;
use tracing::debug;

#[derive(Clone, FromRef, Default)]
pub struct Orchestrator {
    listeners: Arc<Mutex<HashMap<String, Box<dyn Listener + Send>>>>,
    running: Arc<Mutex<HashMap<String, Sender<()>>>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator {
            listeners: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn add_listener(&self, create: CreateListener) {
        let listener: Box<dyn Listener + Send> = match create.r#type {
            ListenerTypes::Http => {
                Box::new(HttpListener::new(create.host, create.port, create.endpoints).await)
                    as Box<dyn Listener + Send>
            }
            ListenerTypes::Https => Box::new(HttpsListener::new()) as Box<dyn Listener + Send>,
            ListenerTypes::Tcp => Box::new(TcpListener::new()) as Box<dyn Listener + Send>,
        };
        let mut listeners = self.listeners.lock().await;

        listeners.entry(create.name).or_insert(listener);

        debug!("Successfully added new listener.");
    }

    pub async fn remove_listener(&self, name: &str) {
        let mut listeners = self.listeners.lock().await;
        self.stop_listener(name).await;
        match listeners.remove(name) {
            Some(_) => debug!("Successfully removed listener {}", name),
            None => debug!("Failed to remove listener {}", name),
        };
    }

    pub async fn start_listener(&self, name: &str) {
        let mut listeners = self.listeners.lock().await;
        if let Some(listener) = listeners.get_mut(name) {
            listener.start().await;
        }
    }
    pub async fn stop_listener(&self, name: &str) {
        let mut listeners = self.listeners.lock().await;
        if let Some(listener) = listeners.get_mut(name) {
            listener.stop().await;
        }
    }

    pub async fn exists(&self, name: &str) -> bool {
        (*(self.listeners.lock().await)).contains_key(name)
    }
}
