use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::models::users::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
    pub token_version: i32,
    pub user_role: UserRole
}