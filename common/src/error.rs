use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("hashing error occured")]
    Hash(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("unauthorized")]
    Auth,
    #[error("encryption error occured")]
    Encrypt(String),
    #[error(transparent)]
    Decode(#[from] hex::FromHexError),
    #[error(transparent)]
    TaskJoin(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Rsa(#[from] rsa::errors::Error),
    #[error(transparent)]
    Pkcs1(#[from] rsa::pkcs1::Error),
}

impl From<aes_gcm::Error> for Error {
    fn from(err: aes_gcm::Error) -> Self {
        Error::Encrypt(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Error::Hash(value.to_string())
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{}", self);

        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let status = match self {
            Error::Hash(_)
            | Error::Sqlx(_)
            | Error::Encrypt(_)
            | Error::Decode(_)
            | Self::TaskJoin(_)
            | Self::Http(_)
            | Self::Rsa(_)
            | Self::Pkcs1(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Auth => StatusCode::UNAUTHORIZED,
        };

        (
            status,
            Json(ErrorResponse {
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
