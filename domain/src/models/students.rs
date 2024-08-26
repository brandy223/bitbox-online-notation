use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStudent {
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = crate::schema::students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatedStudent {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::promotions_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(promotion_id, student_id))]
pub struct PromotionStudent {
    pub promotion_id: Uuid,
    pub student_id: Uuid
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::promotions_students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPromotionStudent {
    pub promotion_id: Uuid,
    pub student_id: Uuid
}