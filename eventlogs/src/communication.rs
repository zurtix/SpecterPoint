use crate::models::{
    config::Config,
    message::{Message, MessageCodec},
};
use futures_util::SinkExt;
use sqlx::SqlitePool;
use std::sync::LazyLock;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub static COMMS: LazyLock<Communication> = LazyLock::new(Communication::new);

#[derive(Clone)]
pub struct Communication {
    sender: Sender<Message>,
}

impl Communication {
    fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { sender: tx.clone() }
    }

    pub async fn start(&self, pool: SqlitePool) {
        let config = envy::from_env::<Config>().expect("Failed to read environment variables");
        let server = format!("{}:{}", config.host, config.event_port);
        let server = TcpListener::bind(server).await.unwrap();
        tokio::spawn(incoming(self.sender.clone(), server, pool));
    }

    pub fn send(&self, message: Message) {
        let _ = self.sender.send(message);
    }
}

pub async fn incoming(tx: Sender<Message>, listener: TcpListener, pool: SqlitePool) {
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let receiver = tx.subscribe();
                tokio::spawn(handle_client(socket, receiver, pool.clone()));
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

pub async fn handle_client(socket: TcpStream, mut receiver: Receiver<Message>, pool: SqlitePool) {
    let mut framed = Framed::new(socket, MessageCodec);

    if let Some(Ok(Message::Auth(username, password))) = framed.next().await {
        if let Ok(user) = common::db::user::get_user(pool, &username).await {
            if let Ok(true) = common::crypt::hash::verify_password_hash(user.password, &password) {
                while let Ok(message) = receiver.recv().await {
                    let _ = framed.send(message).await;
                }
            }
        }
    }
}
