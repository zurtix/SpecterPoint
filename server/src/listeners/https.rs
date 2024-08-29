use super::Listen;
use axum::async_trait;

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
