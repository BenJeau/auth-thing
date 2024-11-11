#[derive(Debug)]
pub enum Error {
    Address(lettre::address::AddressError),
    Lettre(lettre::error::Error),
    Smtp(lettre::transport::smtp::Error),
}

impl From<lettre::address::AddressError> for Error {
    fn from(err: lettre::address::AddressError) -> Self {
        Error::Address(err)
    }
}

impl From<lettre::error::Error> for Error {
    fn from(err: lettre::error::Error) -> Self {
        Error::Lettre(err)
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        Error::Smtp(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Address(e) => e.fmt(f),
            Error::Lettre(e) => e.fmt(f),
            Error::Smtp(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
