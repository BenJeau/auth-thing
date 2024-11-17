pub async fn send_verification_email(
    code: &str,
    recipient: &str,
    mailer: &email::Mailer,
) -> email::Result<email::Response> {
    let body = include_str!("email_templates/verification.html").replace("{code}", code);

    let message = email::MessageBuilder::new()
        .from(format!("Auth Thing <{}>", mailer.username))
        .to(recipient.to_string())
        .subject(format!("{code} is you Auth Thing sign-in code"))
        .body(body)
        .build()?;

    mailer.send(message).await
}
