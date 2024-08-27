use common::{
    crypt::hash::verify_password_hash,
    db::user::get_user,
    models::{
        log::LogMessage,
        user::{BaseCredential, Credentials},
    },
};
use futures_util::SinkExt;
use sqlx::SqlitePool;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{field::Visit, Event};
use tracing_core::{subscriber::Subscriber, Field};
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer,
};

pub struct BroadcastSubscriber {
    sender: broadcast::Sender<LogMessage>,
}

impl BroadcastSubscriber {
    pub fn new(sender: broadcast::Sender<LogMessage>) -> Self {
        Self { sender }
    }
}

impl<S> Layer<S> for BroadcastSubscriber
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = StringVisitor::new();
        event.record(&mut visitor);

        if !visitor.output.is_empty() {
            let _ = self.sender.send(LogMessage {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: metadata.level().to_string(),
                message: visitor.output,
            });
        }
    }
}

struct StringVisitor {
    output: String,
}

impl StringVisitor {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}
impl Visit for StringVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            if !self.output.is_empty() {
                self.output.push_str(", ");
            }
            self.output
                .push_str(&format!("{}: {}", field.name(), value.replace('\n', "")));
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if !self.output.is_empty() {
            self.output.push_str(", ");
        }

        if field.name() == "message" {
            let out = format!("{:?}", value);
            if !out.is_empty() {
                self.output.push_str(&out);
            }
        }
    }
}

pub async fn handle_client(
    socket: TcpStream,
    mut receiver: broadcast::Receiver<LogMessage>,
    pool: SqlitePool,
) {
    let mut framed = Framed::new(socket, LinesCodec::new());

    if let Some(Ok(line)) = framed.next().await {
        if let Ok(creds) = serde_json::from_str::<BaseCredential>(&line) {
            if let Ok(user) = get_user(pool.clone(), &creds.username).await {
                if let Ok(valid) = verify_password_hash(user.password, &creds.password) {
                    if valid {
                        while let Ok(log_message) = receiver.recv().await {
                            if let Ok(message) = serde_json::to_string(&log_message) {
                                let _ = framed.send(message).await;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn log_listen(
    tx: broadcast::Sender<LogMessage>,
    listener: TcpListener,
    pool: SqlitePool,
) {
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let receiver = tx.subscribe();
                tokio::spawn(handle_client(socket, receiver, pool.clone()));
            }
            Err(e) => eprintln!("Failed to accept log connection: {}", e),
        }
    }
}

pub async fn init(host: &str, port: u16, pool: SqlitePool) {
    let (tx, _rx) = broadcast::channel(100);
    let broadcast_subscriber = BroadcastSubscriber::new(tx.clone());

    tracing_subscriber::registry()
        .with(broadcast_subscriber)
        .init();

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    tokio::spawn(log_listen(tx, listener, pool));
}
