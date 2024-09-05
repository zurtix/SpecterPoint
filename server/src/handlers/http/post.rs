use axum::{http::StatusCode, Extension};

pub async fn receive(Extension(key): Extension<Vec<u8>>) -> StatusCode {
    StatusCode::OK
}
