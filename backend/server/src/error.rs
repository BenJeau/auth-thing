use axum::response::{IntoResponse, Response};
use http::StatusCode;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, strum::Display)]
pub enum Error {
    // Generic errors
    NotFound(String),
    Forbidden,
    Unauthorized(String),
    TooManyRequests,
    // Database errors
    Database(database::Error),
    DatabaseMigration(database::MigrateError),
    Io(std::io::Error),
    AddrParse(std::net::AddrParseError),
    // Crypto errors
    ChaCha(chacha20poly1305::Error),
    ChaChaSecretLength,
    PasswordHash(password::PasswordHashError),
    // Auth/User errors
    DisabledUser,
    NotVerified,
    AlreadyVerified,
    InvalidCredentials,
    MailerNotConfigured,
    TotpDisabled,
    TotpSecretNotFound,
    TotpInvalid,
    Totp(totp::Error),
    UnableToValidateToken,
    UnableToCreateToken,
    Jwt(jwt::Error),
    // Password errors
    PasswordRequirements(password::PasswordRequirementsBuilderError),
    PasswordValidation(Vec<password::PasswordError>),
    // Other errors
    SerdeJson(serde_json::Error),
    MissingHeader(String),
    Email(email::Error),
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

impl From<password::PasswordHashError> for Error {
    fn from(e: password::PasswordHashError) -> Self {
        Self::PasswordHash(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<email::Error> for Error {
    fn from(e: email::Error) -> Self {
        Self::Email(e)
    }
}

impl From<totp::Error> for Error {
    fn from(e: totp::Error) -> Self {
        Self::Totp(e)
    }
}

impl From<password::PasswordRequirementsBuilderError> for Error {
    fn from(e: password::PasswordRequirementsBuilderError) -> Self {
        Self::PasswordRequirements(e)
    }
}

impl From<Vec<password::PasswordError>> for Error {
    fn from(e: Vec<password::PasswordError>) -> Self {
        Self::PasswordValidation(e)
    }
}

impl From<jwt::Error> for Error {
    fn from(e: jwt::Error) -> Self {
        Self::Jwt(e)
    }
}

impl IntoResponse for Error {
    #[tracing::instrument(skip_all)]
    fn into_response(self) -> Response {
        error!(error=?self);

        match self {
            Self::Database(database::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS.into_response(),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
            Self::AlreadyVerified => StatusCode::BAD_REQUEST.into_response(),
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
            Self::DisabledUser => {
                (StatusCode::UNAUTHORIZED, "Your account is disabled").into_response()
            }
            Self::NotVerified => {
                (StatusCode::UNAUTHORIZED, "Your account is not verified").into_response()
            }
            Self::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Self::TotpInvalid => (StatusCode::BAD_REQUEST, "Invalid OTP provided").into_response(),
            Self::TotpDisabled => {
                (StatusCode::FORBIDDEN, "2FA is not enabled for this user").into_response()
            }
            Self::TotpSecretNotFound => {
                (StatusCode::NOT_FOUND, "TOTP secret not found for user").into_response()
            }
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED.into_response(),
            Self::MissingHeader(header) => {
                (StatusCode::BAD_REQUEST, format!("Missing header: {header}")).into_response()
            }
            Self::PasswordValidation(requirements) => {
                (StatusCode::BAD_REQUEST, format!("{requirements:?}")).into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
