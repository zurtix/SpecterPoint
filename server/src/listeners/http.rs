use super::Listen;
use crate::handlers::http;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::{async_trait, Router};
use axum_server::Handle;
use common::models::endpoint::Endpoint;
use std::net::SocketAddr;

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
        let addr = self.addr;
        let handle = self.handle.clone();

        for e in self.endpoints.iter() {
            app = app.merge(http::routes(&e.endpoint));
        }

        app = app.route_layer(middleware::from_fn(scatter_header));

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

// TODO: Pull encrypted values that are scattered amongst multiple headers
// TODO: Validate the appropriate header is set based on db values
// TODO Validate that nothing matches the blacklist based on headers
async fn scatter_header(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    // if let Some(current_user) = authorize_current_user(auth_header).await {
    //     // insert the current user into a request extension so the handler can
    //     // extract it
    //     req.extensions_mut().insert(current_user);
    //     Ok(next.run(req).await)
    // } else {
    //     Err(StatusCode::NOT_FOUND)
    // }

    Ok(next.run(req).await)
}
