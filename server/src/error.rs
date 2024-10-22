pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, strum::Display)]
pub enum Error {
    Database(database::Error),
    DatabaseMigration(database::MigrateError),
    Io(std::io::Error),
    AddrParse(std::net::AddrParseError),
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
