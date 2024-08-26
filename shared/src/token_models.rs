use domain::models::tokens::TokenType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
    pub token_version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificClaims {
    pub sub: Uuid,
    #[serde(rename = "type")]
    pub type_: TokenType,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentClaims {
    pub sub: Uuid,
    pub group_id: Uuid,
    pub iat: usize,
    pub exp: usize,
}