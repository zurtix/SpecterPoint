use super::Listen;
use axum::async_trait;

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
