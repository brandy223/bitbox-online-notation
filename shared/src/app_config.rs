
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
}

impl Config {
    pub fn init() -> Config {
        Config {
            database_url: dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expires_in: dotenvy::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN must be set"),
        }
    }
}