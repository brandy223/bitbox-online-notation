use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

use shared::error_models::InternalError;
use shared::error_models::{APIError, ServerError};

#[derive(Debug, Deserialize)]
pub struct MailProps {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub fn send_mail(smtp_transport: &SmtpTransport, message: Message) -> Result<(), APIError> {
    smtp_transport.send(&message)
        .map_err(|_| APIError::ServerError(ServerError::InternalError(InternalError)))?;

    Ok(())
}

pub fn build_mail(mail_props: MailProps) -> Message {
    Message::builder()
        .from(mail_props.from.parse().unwrap())
        .to(mail_props.to.parse().unwrap())
        .subject(mail_props.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(mail_props.body))
        .unwrap()
}