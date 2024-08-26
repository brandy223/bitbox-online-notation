use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::promotions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Promotion {
    pub id: Uuid,
    pub title: String,
    pub start_year: chrono::NaiveDate,
    pub end_year: chrono::NaiveDate,
    pub teacher_id: Uuid,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::promotions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPromotion {
    pub title: String,
    pub start_year: chrono::NaiveDate,
    pub end_year: chrono::NaiveDate,
    pub teacher_id: Uuid,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::promotions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedPromotion {
    pub title: Option<String>,
    pub start_year: Option<chrono::NaiveDate>,
    pub end_year: Option<chrono::NaiveDate>,
}