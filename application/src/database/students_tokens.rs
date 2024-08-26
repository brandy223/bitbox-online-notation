use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::students_tokens::*;
use domain::schema::students_tokens::dsl::students_tokens;
use infrastructure::DBPool;
use uuid::Uuid;

pub fn get_student_token_by_id(conn: &DBPool, id_: Uuid) -> Result<StudentToken, DBError> {
    use domain::schema::students_tokens::dsl::*;

    students_tokens.filter(id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn get_student_token_by_token(conn: &DBPool, token_: String) -> Result<StudentToken, DBError> {
    use domain::schema::students_tokens::dsl::*;

    students_tokens.filter(token.eq(token_))
        .first(&mut conn.get().unwrap())
}

pub fn get_student_tokens_from_student_and_project_id(conn: &DBPool, student_id_: Uuid, project_id_: Uuid) -> Result<StudentToken, DBError> {
    use domain::schema::students_tokens::dsl::*;

    students_tokens.filter(student_id.eq(student_id_))
        .filter(project_id.eq(project_id_))
        .first(&mut conn.get().unwrap())
}

pub fn create_student_token(conn: &DBPool, new_student_token: NewStudentToken) -> Result<Uuid, DBError> {
    use domain::schema::students_tokens::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(students_tokens)
        .values(&new_student_token)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn update_student_token(conn: &DBPool, id_: Uuid, updated_student_token: UpdatedStudentToken) -> Result<(), DBError> {
    use domain::schema::students_tokens::dsl::*;

    // Check if the student exists
    students_tokens.filter(id.eq(id_.clone()))
        .first::<StudentToken>(&mut conn.get().unwrap())?;

    diesel::update(students_tokens.filter(id.eq(id_)))
        .set(&updated_student_token)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_student_token(conn: &DBPool, id_: Uuid) -> Result<(), DBError> {
    use domain::schema::students_tokens::dsl::*;

    // Check if the student exists
    students_tokens.filter(id.eq(id_))
        .first::<StudentToken>(&mut conn.get().unwrap())?;

    diesel::delete(students_tokens.filter(id.eq(id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_all_tokens_from_student(conn: &DBPool, student_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::students_tokens::dsl::*;

    diesel::delete(students_tokens.filter(student_id.eq(student_id_))
    ).execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::projects::test::test_create_project;
    use crate::database::students::test::test_create_student;
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

    fn test_create_student_token() -> (Uuid, Uuid, Uuid) {
        let ctx = TestContext::new();
        let student_id = test_create_student();
        let (project_id, _) = test_create_project();
        let new_student_token = NewStudentToken {
            student_id,
            project_id,
            token: "token".to_string(),
        };

        let result = create_student_token(&ctx.conn, new_student_token).unwrap();
        (result, student_id, project_id)
    }

    #[test]
    fn test_get_student_token_by_id() {
        let (id, _, _) = test_create_student_token();
        let ctx = TestContext::new();
        let result = get_student_token_by_id(&ctx.conn, id).unwrap();
        assert_eq!(result.id, id);
    }

    #[test]
    fn test_get_student_token_by_token() {
        let (_, _, _) = test_create_student_token();
        let ctx = TestContext::new();
        let result = get_student_token_by_token(&ctx.conn, "token".to_string()).unwrap();
        assert_eq!(result.token, "token".to_string());
    }

    #[test]
    fn test_get_student_tokens_from_student_and_project_id() {
        let (_, student_id, project_id) = test_create_student_token();
        let ctx = TestContext::new();
        let result = get_student_tokens_from_student_and_project_id(&ctx.conn, student_id, project_id).unwrap();
        assert_eq!(result.student_id, student_id);
    }

    #[test]
    fn test_update_student_token() {
        let (id, _, _) = test_create_student_token();
        let ctx = TestContext::new();
        let updated_student_token = UpdatedStudentToken {
            used: Some(true),
        };
        update_student_token(&ctx.conn, id, updated_student_token).unwrap();
        let result = get_student_token_by_id(&ctx.conn, id).unwrap();
        assert_eq!(result.used, true);
    }

    #[test]
    fn test_delete_student_token() {
        let (id, _, _) = test_create_student_token();
        let ctx = TestContext::new();
        delete_student_token(&ctx.conn, id).unwrap();
        let result = get_student_token_by_id(&ctx.conn, id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_all_tokens_from_student() {
        let (_, student_id, _) = test_create_student_token();
        let ctx = TestContext::new();
        delete_all_tokens_from_student(&ctx.conn, student_id).unwrap();
        let result = get_student_tokens_from_student_and_project_id(&ctx.conn, student_id, Uuid::new_v4());
        assert!(result.is_err());
    }
}