mod error;
mod mailer;
mod message;

pub use error::{Error, Result};
pub use mailer::Mailer;
pub use message::MessageBuilder;

pub use lettre::transport::smtp::response::Response;
