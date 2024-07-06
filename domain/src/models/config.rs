use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable, ToSchema, Clone)]
#[diesel(table_name = crate::schema::main_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MainConfig {
    pub id: i32,
    pub register: bool,
    pub authorized_domains: Vec<Option<String>>,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::main_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMainConfig {
    pub register: Option<bool>,
    pub authorized_domains: Option<Vec<String>>
}

#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = crate::schema::main_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedMainConfig {
    pub register: Option<bool>,
    pub authorized_domains: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AlertObject {
    pub before_event: bool,
    pub time: chrono::NaiveTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JsonWrapper(serde_json::Value);

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::user_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserConfig {
    pub id: i32,
    pub user_id: Uuid,
    #[schema(value_type = Vec<AlertObject>)]
    pub alerts: Vec<Option<Json>>,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = crate::schema::user_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserConfig {
    pub user_id: Uuid,
    #[schema(value_type = Option<Vec<AlertObject>>)]
    pub alerts: Option<Vec<Json>>,
}

#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = crate::schema::user_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedUserConfig {
    #[schema(value_type = Option<Vec<AlertObject>>)]
    pub alerts: Option<Vec<Json>>,
}