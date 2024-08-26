use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, DbEnum, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[ExistingTypePath = "crate::schema::sql_types::AlertType"]
pub enum AlertType {
    #[db_rename = "started"]
    Started,
    #[db_rename = "pending"]
    Pending,
    #[db_rename = "finished"]
    Finished,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable, Clone)]
#[diesel(table_name = crate::schema::done_alerts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DoneAlert {
    pub id: i32,
    pub description: Option<String>,
    pub project_id: Uuid,
    #[serde(rename = "type")]
    pub type_: AlertType,
    pub published_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::done_alerts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewDoneAlert {
    pub description: Option<String>,
    pub project_id: Uuid,
    #[serde(rename = "type")]
    pub type_: AlertType,
}