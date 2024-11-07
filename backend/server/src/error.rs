use axum::response::{IntoResponse, Response};
use http::StatusCode;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, strum::Display)]
pub enum Error {
    Database(database::Error),
    DatabaseMigration(database::MigrateError),
    Io(std::io::Error),
    AddrParse(std::net::AddrParseError),
    NotFound(String),
    ChaCha(chacha20poly1305::Error),
    ChaChaSecretLength,
    Argon2PasswordHash(argon2::password_hash::Error),
    DisabledUser,
    InvalidCredentials,
    Jsonwebtoken(jsonwebtoken::errors::Error),
    SerdeJson(serde_json::Error),
    Forbidden,
    Unauthorized(String),
    MissingHeader(String),
}

impl std::error::Error for Error {}

impl From<database::Error> for Error {
    fn from(e: database::Error) -> Self {
        Self::Database(e)
    }
}

impl From<database::MigrateError> for Error {
    fn from(e: database::MigrateError) -> Self {
        Self::DatabaseMigration(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::AddrParse(e)
    }
}

impl From<chacha20poly1305::Error> for Error {
    fn from(e: chacha20poly1305::Error) -> Self {
        Self::ChaCha(e)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::Argon2PasswordHash(e)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::Jsonwebtoken(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl IntoResponse for Error {
    #[tracing::instrument(skip_all)]
    fn into_response(self) -> Response {
        error!(error=?self);

        match self {
            Self::Database(database::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
            Self::DisabledUser => {
                (StatusCode::UNAUTHORIZED, "Your account is disabled").into_response()
            }
            Self::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Self::Database(database::Error::Database(err)) if err.code() == Some("2067".into()) => {
                StatusCode::CONFLICT.into_response()
            }
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED.into_response(),
            Self::MissingHeader(header) => {
                (StatusCode::BAD_REQUEST, format!("Missing header: {header}")).into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
