use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::promotions::{NewPromotion, Promotion, UpdatedPromotion};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use infrastructure::DBPool;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PromotionSearchParams {
    title: Option<String>,
    start_year: Option<NaiveDate>,
    end_year: Option<NaiveDate>,
}

pub fn get_all_promotions_from_teacher_id(conn: &DBPool, teacher_id_: Uuid) -> Result<Vec<Promotion>, DBError> {
    use domain::schema::promotions::dsl::*;

    promotions.filter(teacher_id.eq(teacher_id_))
        .load::<Promotion>(&mut conn.get().unwrap())
}

pub fn get_promotion_by_id(conn: &DBPool, _id: Uuid) -> Result<Promotion, DBError> {
    use domain::schema::promotions::dsl::*;

    promotions.filter(id.eq(_id))
        .first(&mut conn.get().unwrap())
}

pub fn get_promotions_by_matching_date_and_title(conn: &DBPool, params: &PromotionSearchParams, teacher_id_: Uuid) -> Result<Vec<Promotion>, DBError> {
    use domain::schema::promotions::dsl::*;

    let mut query = promotions.into_boxed();

    if let Some(_title) = &params.title {
        query = query.filter(title.ilike(_title));
    }

    if let Some(_start_year) = params.start_year {
        query = query.filter(start_year.ge(_start_year));
    }

    if let Some(_end_year) = params.end_year {
        query = query.filter(end_year.le(_end_year));
    }

    query.filter(teacher_id.eq(teacher_id_))
        .load::<Promotion>(&mut conn.get().unwrap())
}

pub fn create_promotion(conn: &DBPool, new_promotion: NewPromotion) -> Result<Uuid, DBError> {
    use domain::schema::promotions::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(promotions)
        .values(&new_promotion)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn update_promotion(conn: &DBPool, _id: Uuid, update_promotion: UpdatedPromotion) -> Result<(), DBError> {
    use domain::schema::promotions::dsl::*;

    diesel::update(promotions.filter(id.eq(_id)))
        .set(&update_promotion)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_promotion(conn: &DBPool, _id: Uuid) -> Result<(), DBError> {
    use domain::schema::promotions::dsl::*;

    diesel::delete(promotions.filter(id.eq(_id)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::database::users::tests::test_create_user;
    use domain::models::promotions::{NewPromotion, UpdatedPromotion};
    use infrastructure::init_pool;

    struct TestContext {
        conn: DBPool,
    }
    
    impl TestContext {
        fn new() -> Self {
            TestContext {
                conn: init_pool(
                    dotenvy::var("DATABASE_URL")
                        .expect("DATABASE_URL must be set")
                        .as_str()
                ),
            }
        }
    }

    pub fn test_create_promotion() -> (Uuid, Uuid) {
        let context = TestContext::new();

        let teacher_id = test_create_user();
        let new_promotion = NewPromotion {
            title: "test".to_string(),
            start_year: NaiveDate::from_ymd(2021, 1, 1),
            end_year: NaiveDate::from_ymd(2021, 12, 31),
            teacher_id,
        };
    
        (create_promotion(&context.conn, new_promotion).unwrap(), teacher_id)
    }

    #[test]
    fn test_get_all_promotions_from_teacher_id() {
        let context = TestContext::new();

        let (_, teacher_id) = test_create_promotion();

        get_all_promotions_from_teacher_id(&context.conn, teacher_id).unwrap();
    }

    #[test]
    fn test_get_promotion_by_id() {
        let context = TestContext::new();

        let (id, _) = test_create_promotion();

        get_promotion_by_id(&context.conn, id).unwrap();
    }

    #[test]
    fn test_get_promotions_by_matching_date_and_title() {
        let context = TestContext::new();

        let (_, teacher_id) = test_create_promotion();

        get_promotions_by_matching_date_and_title(&context.conn, &PromotionSearchParams {
            title: Some("test".to_string()),
            start_year: Some(NaiveDate::from_ymd(2021, 1, 1)),
            end_year: Some(NaiveDate::from_ymd(2021, 12, 31))
        }, teacher_id).unwrap();
    }

    #[test]
    fn test_update_promotion() {
        let context = TestContext::new();

        let (id, _) = test_create_promotion();

        let updated_promotion = UpdatedPromotion {
            title: Some("Re-test".to_string()),
            start_year: None,
            end_year: None,
        };

        update_promotion(&context.conn, id, updated_promotion).unwrap();
    }

    #[test]
    fn test_delete_promotion() {
        let context = TestContext::new();

        let (id, _) = test_create_promotion();

        delete_promotion(&context.conn, id).unwrap();
    }
}