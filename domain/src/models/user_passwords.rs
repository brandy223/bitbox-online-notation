use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::user_passwords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id))]
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