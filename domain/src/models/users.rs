use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    #[db_rename = "admin"]
    Admin,
    #[db_rename = "user"]
    User,
}

#[derive(Debug, Clone, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub has_validated_email: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub role: UserRole,
    pub token_version: i32,
}

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub has_validated_email: Option<bool>,
    pub role: Option<UserRole>,
    pub token_version: Option<i32>,
}

#[derive(Debug, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedUserInfo {
    pub username: Option<String>,
    pub email: Option<String>,
}