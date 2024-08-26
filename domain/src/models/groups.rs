use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub mark: Option<f64>,
    pub max_mark: i32,
    pub project_id: Uuid,
}

#[derive(Debug, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroup {
    pub name: String,
    pub project_id: Uuid,
    pub max_mark: Option<i32>,
}

#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedGroup {
    pub name: Option<String>,
    pub mark: Option<f64>,
    pub max_mark: Option<i32>,
}

#[derive(Debug, Serialize, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crate::schema::groups_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(group_id, student_id))]
pub struct GroupStudent {
    pub group_id: Uuid,
    pub student_id: Uuid,
    pub student_mark: Option<f64>,
    pub max_mark: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::groups_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroupStudent {
    pub group_id: Uuid,
    pub student_id: Uuid,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::groups_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedGroupStudent {
    pub student_mark: Option<f64>,
}