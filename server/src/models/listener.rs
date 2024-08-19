use axum::{async_trait, routing::get, Router};
use axum_server::Handle;
use common::models::endpoint::Endpoint;
use std::net::SocketAddr;

#[async_trait]
pub trait Listen {
    async fn start(&mut self);
    async fn stop(&mut self);
}

pub struct HttpListener {
    addr: SocketAddr,
    endpoints: Vec<Endpoint>,
    handle: Handle,
}

impl HttpListener {
    pub async fn new(host: String, port: u16, endpoints: Vec<Endpoint>) -> Self {
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        Self {
            addr,
            endpoints,
            handle: Handle::new(),
        }
    }
}

#[async_trait]
impl Listen for HttpListener {
    async fn start(&mut self) {
        let mut app = Router::new();

        for e in self.endpoints.iter() {
            app = app.route(&e.endpoint, get(crate::handlers::get::handle_get_http));
        }

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
impl Listen for HttpsListener {
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
impl Listen for TcpListener {
    async fn start(&mut self) {
        !unimplemented!()
    }
    async fn stop(&mut self) {
        !unimplemented!()
    }
}
