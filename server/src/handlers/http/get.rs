use axum::{http::StatusCode, Extension};
use common::crypt::aes;
use eventlogs::{agent, models::agent::Agent};

pub async fn check_in(Extension((key, data)): Extension<(String, String)>) -> StatusCode {
    let raw = aes::decrypt(&key, &data).map_err(|_| StatusCode::NOT_FOUND);
    let id = "test".to_string();
    let agent = Agent { id };
    agent!(agent);

    StatusCode::OK
}
