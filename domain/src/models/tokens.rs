use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[ExistingTypePath = "crate::schema::sql_types::TokenType"]
pub enum TokenType {
    #[db_rename = "pass-reset"]
    PassReset,
    #[db_rename = "account-activation"]
    AccountActivation,
    #[db_rename = "email-verification"]
    EmailVerification,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: Uuid,
    pub token: String,
    #[serde(rename = "type")]
    pub type_: TokenType,
    pub used: bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewToken {
    pub token: String,
    #[serde(rename = "type")]
    pub type_: TokenType,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedToken {
    #[serde(rename = "type")]
    pub type_: Option<TokenType>,
    pub used: Option<bool>
}