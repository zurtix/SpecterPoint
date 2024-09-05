use futures_util::SinkExt;
use rand::Rng;
use std::{collections::HashMap, sync::Arc};
use tauri::{AppHandle, Manager};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
};

use crate::models::{
    log::Log,
    message::{Message, MessageCodec},
};

#[derive(Clone, Default)]
pub struct EventManager {
    connections: Arc<Mutex<HashMap<i64, Sender<()>>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn connect(&self, connection: Connection) {
        if self.connections.lock().await.contains_key(&connection.id) {
            return;
        }

        let (tx, mut rx) = mpsc::channel(1);

        self.connections.lock().await.insert(connection.id, tx);

        tokio::spawn(async move {
            if let Ok(socket) = TcpStream::connect(connection.server).await {
                let mut framed = Framed::new(socket, MessageCodec);

                let auth = Message::Auth(connection.username, connection.password);

                let _ = framed.send(auth).await;

                let msg = Message::Log(Log {
                    level: "DEBUG".to_string(),
                    message: format!("Event manager connected to server {}", connection.id),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });

                let _ = connection.handle.emit_all("event", msg);

                loop {
                    tokio::select! {
                        _ = rx.recv() =>  break,
                        Some(Ok(message)) = framed.next() => {
                            let _ = connection.handle.emit_all("event", message);
                        }
                    }
                }
            }
        });
    }

    pub async fn disconnect(&self, id: &i64) {
        if let Some(tx) = self.connections.lock().await.remove(id) {
            let _ = tx.send(()).await;
        }
    }
}

pub struct Connection {
    pub handle: AppHandle,
    pub username: String,
    pub password: String,
    pub server: String,
    pub id: i64,
}

pub struct ConnectionBuilder {
    pub handle: AppHandle,
    pub username: Option<String>,
    pub password: Option<String>,
    pub server: Option<String>,
    pub id: Option<i64>,
}

impl ConnectionBuilder {
    pub fn new(handle: AppHandle) -> Self {
        Self {
            handle,
            username: None,
            password: None,
            server: None,
            id: None,
        }
    }

    pub fn auth(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    pub fn server(mut self, server: String) -> Self {
        self.server = Some(server);
        self
    }

    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self) -> Connection {
        let mut rng = rand::thread_rng();

        Connection {
            handle: self.handle,
            username: self.username.unwrap_or_default(),
            password: self.password.unwrap_or_default(),
            server: self.server.unwrap_or_default(),
            id: self.id.unwrap_or(rng.gen()),
        }
    }
}
