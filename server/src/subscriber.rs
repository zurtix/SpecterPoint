use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tracing::{field::Visit, Event};
use tracing_core::{subscriber::Subscriber, Field};
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer,
};

pub struct BroadcastSubscriber {
    sender: broadcast::Sender<String>,
}

impl BroadcastSubscriber {
    pub fn new(sender: broadcast::Sender<String>) -> Self {
        Self { sender }
    }
}

pub async fn handle_client(mut socket: TcpStream, mut receiver: broadcast::Receiver<String>) {
    while let Ok(log_message) = receiver.recv().await {
        if let Err(e) = socket.write_all(log_message.as_bytes()).await {
            eprintln!("Failed to send message to client: {}", e);
            return;
        }
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

        let message = format!(
            "{} {}: {}\n",
            chrono::Utc::now().to_rfc3339(),
            metadata.level(),
            visitor.output
        );

        let _ = self.sender.send(message);
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
        if !self.output.is_empty() {
            self.output.push_str(", ");
        }
        self.output
            .push_str(&format!("{}: {}", field.name(), value.replace('\n', "")));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if !self.output.is_empty() {
            self.output.push_str(", ");
        }
        self.output
            .push_str(&format!("{}: {:?}", field.name(), value));
    }
}

pub async fn log_listen(tx: broadcast::Sender<String>, listener: TcpListener) {
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let receiver = tx.subscribe();
                tokio::spawn(handle_client(socket, receiver));
            }
            Err(e) => eprintln!("Failed to accept log connection: {}", e),
        }
    }
}

pub async fn init() {
    let (tx, _rx) = broadcast::channel(100);
    let broadcast_subscriber = BroadcastSubscriber::new(tx.clone());

    tracing_subscriber::registry()
        .with(broadcast_subscriber)
        .init();

    let listener = TcpListener::bind("localhost:8081").await.unwrap();
    tokio::spawn(log_listen(tx, listener));
}
