use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::marks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(group_id, noted_student_id, grader_student_id))]
pub struct Mark {
    pub project_id: Uuid,
    pub group_id: Uuid,
    pub noted_student_id: Uuid,
    pub grader_student_id: Uuid,
    pub mark: f64,
    pub max_mark: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::marks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMark {
    pub project_id: Uuid,
    pub group_id: Uuid,
    pub noted_student_id: Uuid,
    pub grader_student_id: Uuid,
    pub mark: f64,
    pub max_mark: Option<i32>,
    pub comment: Option<String>,
}