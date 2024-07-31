#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hasing problem: {}", .0)]
    Hash(String),
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error("{}", .0)]
    Backend(String),
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Error::Hash(value.to_string())
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
