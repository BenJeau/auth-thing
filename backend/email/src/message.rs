use lettre::message::header::ContentType;

pub struct MessageBuilder {
    pub from: Option<String>,
    pub to: Option<String>,
    pub subject: Option<String>,
    pub body: String,
    pub content_type: ContentType,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
            subject: None,
            body: "".to_string(),
            content_type: ContentType::TEXT_HTML,
        }
    }

    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }

    pub fn to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    pub fn is_html(mut self) -> Self {
        self.content_type = ContentType::TEXT_HTML;
        self
    }

    pub fn is_plain(mut self) -> Self {
        self.content_type = ContentType::TEXT_PLAIN;
        self
    }

    pub fn build(self) -> crate::error::Result<lettre::Message> {
        let mut builder = lettre::Message::builder().header(self.content_type);

        if let Some(from) = self.from {
            builder = builder.from(from.parse()?);
        }

        if let Some(to) = self.to {
            builder = builder.to(to.parse()?);
        }

        if let Some(subject) = self.subject {
            builder = builder.subject(subject);
        }

        builder.body(self.body).map_err(Into::into)
    }
}
