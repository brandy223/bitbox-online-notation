use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::mfa_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MfaCode {
    pub id: Uuid,
    pub code: String,
    pub iat: NaiveDateTime,
    pub exp: NaiveDateTime,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::mfa_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMfaCode {
    pub user_id: Uuid,
    pub code: String,
    pub exp: Option<NaiveDateTime>,
}