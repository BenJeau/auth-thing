use lettre::{
    transport::smtp::{authentication::Credentials, response::Response},
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

pub struct Mailer {
    smtp: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
    pub fn new(relay: &str, username: &str, password: &str) -> crate::error::Result<Self> {
        let creds = Credentials::new(username.to_owned(), password.to_owned());

        let smtp = AsyncSmtpTransport::<Tokio1Executor>::relay(relay)?
            .credentials(creds)
            .build();

        Ok(Self { smtp })
    }

    pub async fn send(&self, message: lettre::Message) -> crate::error::Result<Response> {
        self.smtp.send(message).await.map_err(Into::into)
    }
}
