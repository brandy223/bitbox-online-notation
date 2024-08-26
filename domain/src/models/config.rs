use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use diesel_as_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};
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

// #[sql_type(crate::schema::sql_types::alert)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, AsJsonb)]
pub struct Alert {
    pub before_event: bool,
    pub hours: i8,
}

#[derive(Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::user_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserConfig {
    pub id: i32,
    pub user_id: Uuid,
    pub alerts: Vec<Option<Alert>>,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = crate::schema::user_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserConfig {
    pub user_id: Uuid,
    pub alerts: Option<Vec<Alert>>,
}

#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = crate::schema::user_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedUserConfig {
    pub alerts: Option<Vec<Alert>>,
}