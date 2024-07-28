use axum::{async_trait, routing::get, Router};
use axum_server::Handle;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[async_trait]
pub trait Listener {
    async fn start(&mut self);
    async fn stop(&mut self);
}

pub struct HttpListener {
    addr: SocketAddr,
    endpoints: Option<Vec<String>>,
    handle: Handle,
}

impl HttpListener {
    pub async fn new(host: String, port: u16, endpoints: Option<Vec<String>>) -> Self {
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        Self {
            addr,
            endpoints,
            handle: Handle::new(),
        }
    }
}

#[async_trait]
impl Listener for HttpListener {
    async fn start(&mut self) {
        let app = Router::new().route("/", get(crate::handlers::get::handle_temp));

        let addr = self.addr;

        let handle = self.handle.clone();
        tokio::spawn(async move {
            axum_server::bind(addr)
                .handle(handle)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
    }

    async fn stop(&mut self) {
        self.handle.shutdown();
    }
}

#[derive(Clone)]
pub struct HttpsListener {}

impl HttpsListener {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Listener for HttpsListener {
    async fn start(&mut self) {
        !unimplemented!()
    }
    async fn stop(&mut self) {
        !unimplemented!()
    }
}

#[derive(Clone)]
pub struct TcpListener {}

impl TcpListener {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Listener for TcpListener {
    async fn start(&mut self) {
        !unimplemented!()
    }
    async fn stop(&mut self) {
        !unimplemented!()
    }
}
