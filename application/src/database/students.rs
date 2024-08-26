use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::students::*;
use infrastructure::DBPool;
use uuid::Uuid;

pub fn get_student_by_id(conn: &DBPool, id_: Uuid) -> Result<Student, DBError> {
    use domain::schema::students::dsl::*;

    students.filter(id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn get_students_from_promotion_id(conn: &DBPool, promotion_id_: Uuid) -> Result<Vec<Student>, DBError> {
    use domain::schema::students::dsl::*;
    use domain::schema::promotions_students::dsl::*;

    let students_ids = promotions_students
        .filter(promotion_id.eq(promotion_id_))
        .select(student_id)
        .load::<Uuid>(&mut conn.get().unwrap())?;

    students.filter(id.eq_any(students_ids))
        .load::<Student>(&mut conn.get().unwrap())
}

pub fn create_student(conn: &DBPool, new_student: NewStudent) -> Result<Uuid, DBError> {
    use domain::schema::students::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(students)
        .values(&new_student)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn create_promotion_students(conn: &DBPool, new_promotion_students: Vec<NewPromotionStudent>) -> Result<(), DBError> {
    use domain::schema::promotions_students::dsl::*;

    diesel::insert_into(promotions_students)
        .values(&new_promotion_students)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn update_student(conn: &DBPool, id_: Uuid, updated_student: UpdatedStudent) -> Result<(), DBError> {
    use domain::schema::students::dsl::*;

    // Check if the student exists
    students.filter(id.eq(id_.clone()))
        .first::<Student>(&mut conn.get().unwrap())?;

    diesel::update(students.filter(id.eq(id_)))
        .set(&updated_student)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_student(conn: &DBPool, id_: Uuid) -> Result<(), DBError> {
    use domain::schema::students::dsl::*;

    // Check if the student exists
    students.filter(id.eq(id_.clone()))
        .first::<Student>(&mut conn.get().unwrap())?;

    diesel::delete(students.filter(id.eq(id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_promotion_student(conn: &DBPool, promotion_id_: Uuid, student_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::promotions_students::dsl::*;

    diesel::delete(promotions_students
        .filter(promotion_id.eq(promotion_id_))
        .filter(student_id.eq(student_id_))
    ).execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn remove_student_from_all_promotions(conn: &DBPool, student_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::promotions_students::dsl::*;

    diesel::delete(promotions_students.filter(student_id.eq(student_id_))
    ).execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::database::promotions::tests::test_create_promotion;
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

    pub fn test_create_student() -> Uuid {
        let context = TestContext::new();

        let random = Uuid::new_v4();
        let new_student = NewStudent{
            name: format!("test-{}", random),
            surname: format!("test-{}", random),
            email: "none".to_string(),
        };

        create_student(&context.conn, new_student).unwrap()
    }

    fn test_create_promotion_student() -> (Uuid, Uuid) {
        let context = TestContext::new();

        let (promotion_id, _) = test_create_promotion();
        let student_id = test_create_student();

        let new_promotion_student = vec![NewPromotionStudent {
            promotion_id,
            student_id
        }];

        create_promotion_students(&context.conn, new_promotion_student).unwrap();

        (promotion_id, student_id)
    }

    #[test]
    fn test_get_student_by_id() {
        let context = TestContext::new();

        let student_id = test_create_student();

        let student = get_student_by_id(&context.conn, student_id).unwrap();
        assert_eq!(student_id, student.id)
    }

    #[test]
    fn test_get_student_by_promotion_id() {
        let context = TestContext::new();

        let (promotion_id, student_id) = test_create_promotion_student();

        let students = get_students_from_promotion_id(&context.conn, promotion_id).unwrap();
        assert_eq!(students[0].id, student_id);
    }

    #[test]
    fn test_update_student() {
        let context = TestContext::new();

        let student_id = test_create_student();

        let random = Uuid::new_v4();
        let updated_student = UpdatedStudent {
            name: Some(format!("updated-{}", random)),
            surname: Some(format!("updated-{}", random)),
            email: None,
        };
        update_student(&context.conn, student_id, updated_student).unwrap();

        let student = get_student_by_id(&context.conn, student_id).unwrap();
        assert_eq!(student.name, format!("updated-{}", random));
    }

    #[test]
    fn test_delete_student() {
        let context = TestContext::new();

        let student_id = test_create_student();

        delete_student(&context.conn, student_id).unwrap();
    }

    #[test]
    fn test_delete_promotion_student() {
        let context = TestContext::new();

        let (promotion_id, student_id) = test_create_promotion_student();

        delete_promotion_student(&context.conn, promotion_id, student_id).unwrap();
    }

    #[test]
    fn test_remove_student_from_all_promotions() {
        let context = TestContext::new();

        let student_id = test_create_student();

        remove_student_from_all_promotions(&context.conn, student_id).unwrap();
    }
}