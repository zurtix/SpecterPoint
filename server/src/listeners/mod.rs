use axum::async_trait;

pub mod http;
pub mod https;
pub mod tcp;

#[async_trait]
pub trait Listen {
    async fn start(&mut self);
    async fn stop(&mut self);
}
