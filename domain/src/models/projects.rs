use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, ToSchema)]
#[ExistingTypePath = "crate::schema::sql_types::ProjectState"]
pub enum ProjectState {
    #[db_rename = "not-started"]
    NotStarted,
    #[db_rename = "in-progress"]
    InProgress,
    #[db_rename = "finished"]
    Finished,
    #[db_rename = "notation-finished"]
    NotationFinished,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub notation_period_duration: i32,
    pub promotion_id: Uuid,
    pub state: ProjectState,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProject {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: NaiveDateTime,
    pub notation_period_duration: Option<i32>,
    pub promotion_id: Uuid,
    pub state: Option<ProjectState>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub notation_period_duration: Option<i32>,
    pub state: Option<ProjectState>,
}