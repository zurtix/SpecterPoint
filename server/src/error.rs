use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error(StatusCode, String, Option<serde_json::Value>);

impl Error {
    pub fn new(code: u16, message: &str) -> Self {
        let c = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self(c, message.to_string(), None)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!({
        "code": self.0.as_u16(),
        "message": self.1,
        "extra": self.2
        }));

        (self.0, body).into_response()
    }
}
