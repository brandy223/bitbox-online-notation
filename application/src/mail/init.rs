use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

use shared::app_config::Config;
use shared::error_models::{APIError, InternalError, ServerError};

pub fn init_smtp_client(config: &Config) -> SmtpTransport {
    let creds = Credentials::new(config.smtp_config.username.clone(), config.smtp_config.password.clone());
    SmtpTransport::relay(&config.smtp_config.host)
        .map_err(|_| APIError::ServerError(ServerError::InternalError(InternalError)))
        .unwrap().credentials(creds)
        .port(config.smtp_config.port)
        .build()
}