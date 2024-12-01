#[derive(Debug)]
pub enum Error {
    Rsa(rsa::Error),
    Pkcs1(rsa::pkcs1::Error),
    RingUnspecified(ring::error::Unspecified),
    RingKeyRejected(ring::error::KeyRejected),
    Jwt(jsonwebtoken::errors::Error),
    InvalidAlgorithm(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<rsa::Error> for Error {
    fn from(err: rsa::Error) -> Error {
        Error::Rsa(err)
    }
}

impl From<rsa::pkcs1::Error> for Error {
    fn from(err: rsa::pkcs1::Error) -> Error {
        Error::Pkcs1(err)
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(err: ring::error::Unspecified) -> Error {
        Error::RingUnspecified(err)
    }
}

impl From<ring::error::KeyRejected> for Error {
    fn from(err: ring::error::KeyRejected) -> Error {
        Error::RingKeyRejected(err)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Error {
        Error::Jwt(err)
    }
}
