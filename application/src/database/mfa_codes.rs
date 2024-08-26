use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::mfa_codes::*;
use infrastructure::DBPool;
use uuid::Uuid;

pub fn get_mfa_code_by_id(conn: &DBPool, id_: Uuid) -> Result<MfaCode, DBError> {
    use domain::schema::mfa_codes::dsl::*;

    mfa_codes.filter(id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn create_mfa_code(conn: &DBPool, new_mfa_code: NewMfaCode) -> Result<Uuid, DBError> {
    use domain::schema::mfa_codes::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(mfa_codes)
        .values(&new_mfa_code)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn delete_mfa_code_by_id(conn: &DBPool, id_: Uuid) -> Result<(), DBError> {
    use domain::schema::mfa_codes::dsl::*;

    diesel::delete(mfa_codes.filter(id.eq(id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_mfa_code_by_user_id(conn: &DBPool, user_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::mfa_codes::dsl::*;

    diesel::delete(mfa_codes.filter(user_id.eq(user_id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::authentication::codes::generate_random_code;
    use crate::database::users::tests::test_create_user;
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

    fn test_create_mfa_code() -> (Uuid, Uuid) {
        let context = TestContext::new();

        let user_id = test_create_user();
        let code = generate_random_code(6);
        let new_mfa_code = NewMfaCode {
            user_id,
            code,
            exp: None,
        };
        (create_mfa_code(&context.conn, new_mfa_code).unwrap(), user_id)
    }

    #[test]
    fn test_get_mfa_code_by_id() {
        let (id, _) = test_create_mfa_code();
        let context = TestContext::new();
        let mfa_code = get_mfa_code_by_id(&context.conn, id).unwrap();
        assert_eq!(mfa_code.id, id);
    }

    #[test]
    fn test_delete_mfa_code_by_id() {
        let (id, _) = test_create_mfa_code();
        let context = TestContext::new();
        delete_mfa_code_by_id(&context.conn, id).unwrap();
        let result = get_mfa_code_by_id(&context.conn, id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_mfa_code_by_user_id() {
        let (_, user_id) = test_create_mfa_code();
        let context = TestContext::new();
        delete_mfa_code_by_user_id(&context.conn, user_id).unwrap();
        let result = get_mfa_code_by_id(&context.conn, user_id);
        assert!(result.is_err());
    }
}