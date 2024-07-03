use diesel::prelude::*;
use diesel::result::Error as DBError;
use uuid::Uuid;

use domain::models::user_passwords::*;
use infrastructure::DBPool;

pub fn get_user_password_by_user_id(conn: &DBPool, _user_id: Uuid) -> Result<UserPassword, DBError> {
    use domain::schema::user_passwords::dsl::*;

    user_passwords.filter(user_id.eq(_user_id))
        .first(&mut conn.get().unwrap())
}

pub fn create_user_password(conn: &DBPool, new_user_password: NewUserPassword) -> Result<(), DBError> {
    use domain::schema::user_passwords::dsl::*;

    diesel::insert_into(user_passwords)
        .values(&new_user_password)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn update_user_password(conn: &DBPool, _user_id: Uuid, update_user_password: UpdatedUserPassword) -> Result<(), DBError> {
    use domain::schema::user_passwords::dsl::*;

    diesel::update(user_passwords.filter(user_id.eq(_user_id)))
        .set(&update_user_password)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_user_password(conn: &DBPool, _user_id: Uuid) -> Result<(), DBError> {
    use domain::schema::user_passwords::dsl::*;

    diesel::delete(user_passwords.filter(user_id.eq(_user_id)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use infrastructure::init_pool;
    use crate::database::users::tests::test_create_user;

    use super::*;

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

    fn test_create_user_password() -> Uuid {
        let context = TestContext::new();

        let user_id = test_create_user();
        let new_user_password = NewUserPassword {
            user_id,
            password: "password".to_string(),
        };

        create_user_password(&context.conn, new_user_password).unwrap();
        user_id
    }

    #[test]
    fn test_get_user_password_by_user_id() {
        let context = TestContext::new();

        let user_id = test_create_user_password();
        let user_password = get_user_password_by_user_id(&context.conn, user_id).unwrap();
        assert_eq!(user_password.user_id, user_id);
    }

    #[test]
    fn test_update_user_password() {
        let context = TestContext::new();

        let user_id = test_create_user_password();

        let updated_user_password = UpdatedUserPassword {
            password: Some("new_password".to_string()),
        };

        update_user_password(&context.conn, user_id, updated_user_password).unwrap();
        let user_password = get_user_password_by_user_id(&context.conn, user_id).unwrap();
        assert_eq!(user_password.password, "new_password");
    }

    #[test]
    fn test_delete_user_password() {
        let context = TestContext::new();

        let user_id = test_create_user_password();
        delete_user_password(&context.conn, user_id).unwrap();
    }
}