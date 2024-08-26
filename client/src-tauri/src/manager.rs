use common::models::log::LogMessage;
use common::models::user::Credentials;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Manager;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Clone)]
pub struct TcpManager {
    connections: Arc<Mutex<HashMap<i64, mpsc::Sender<()>>>>,
}

impl TcpManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_connection(
        &self,
        creds: Credentials,
        id: i64,
        addr: SocketAddr,
        app_handle: AppHandle,
    ) {
        if self.connections.lock().unwrap().get(&id).is_some() {
            return;
        }

        let (tx, mut rx) = mpsc::channel(1);

        self.connections.lock().unwrap().insert(id, tx);

        let handle_clone = app_handle.clone();

        tokio::spawn(async move {
            if let Ok(socket) = TcpStream::connect(addr).await {
                let mut framed = Framed::new(socket, LinesCodec::new());

                if let Ok(auth) = serde_json::to_string(&creds) {
                    let _ = framed.send(auth).await;

                    let _ = handle_clone.emit_all(
                        "log-event",
                        LogMessage {
                            timestamp: chrono::Utc::now().to_rfc3339(),
                            level: "INFO".to_string(),
                            message: format!("Server {} connected.", id),
                        },
                    );

                    loop {
                        tokio::select! {
                            _ = rx.recv() => break,
                            Some(Ok(line)) = framed.next() => {
                                if let Ok(log) = serde_json::from_str::<LogMessage>(&line) {
                                    let _ = handle_clone.emit_all("log-event", log);
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn remove_connection(&self, id: i64) {
        if let Some(tx) = self.connections.lock().unwrap().remove(&id) {
            let _ = tx.send(());
        }
    }
}
