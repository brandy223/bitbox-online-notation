use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_passwords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserPassword {
    pub user_id: Uuid,
    pub password: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = crate::schema::user_passwords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserPassword {
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Debug, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::user_passwords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedUserPassword {
    pub password: Option<String>,
}