use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::students_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentToken {
    pub id: Uuid,
    pub token: String,
    pub student_id: Uuid,
    pub project_id: Uuid,
    pub used: bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::students_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStudentToken {
    pub token: String,
    pub student_id: Uuid,
    pub project_id: Uuid,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::students_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedStudentToken {
    #[serde(rename = "type")]
    pub used: Option<bool>
}