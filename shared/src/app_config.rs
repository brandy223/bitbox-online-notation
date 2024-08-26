use domain::models::config::MainConfig;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: String,
    pub password_reset_expires_in: String,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_config: JwtConfig,
    pub smtp_config: SmtpConfig,
    pub main_config: MainConfig,
}

impl Config {
    pub fn init(main_config: MainConfig) -> Config {
        Config {
            database_url: dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_config: JwtConfig {
                secret: dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                expires_in: dotenvy::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN must be set"),
                password_reset_expires_in: dotenvy::var("JWT_PASSWORD_RESET_EXPIRES_IN").expect("JWT_PASSWORD_RESET_EXPIRES_IN must be set"),
            },
            smtp_config: SmtpConfig {
                host: dotenvy::var("SMTP_HOST").expect("SMTP_HOST must be set"),
                port: dotenvy::var("SMTP_PORT").expect("SMTP_PORT must be set").parse().unwrap(),
                username: dotenvy::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
                password: dotenvy::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            },
            main_config,
        }
    }
}