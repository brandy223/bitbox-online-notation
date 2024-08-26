use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use shared::app_config::Config;
use shared::error_models::{APIError, InternalError, ServerError, UnauthorizedError, UserError};

pub fn encode_token<T: Serialize>(token: &T, app_config: &Config) -> Result<String, APIError> {
    let header = Header::new(Algorithm::HS512);
    encode(&header, token, &EncodingKey::from_secret(app_config.jwt_config.secret.as_bytes()))
        .map_err(|_| APIError::ServerError(ServerError::InternalError(InternalError)))
}

pub fn decode_token<T: DeserializeOwned>(token: &str, app_config: &Config) -> Result<T, APIError> {
    let validation = Validation::new(Algorithm::HS512);
    let token_data = decode::<T>(token, &DecodingKey::from_secret(app_config.jwt_config.secret.as_bytes()), &validation)
        .map_err(|_| APIError::UserError(UserError::Unauthorized(UnauthorizedError)))?;

    Ok(token_data.claims)
}