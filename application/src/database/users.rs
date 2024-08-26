use diesel::prelude::*;
use diesel::result::Error as DBError;
use uuid::Uuid;

use domain::models::users::*;
use infrastructure::DBPool;

pub fn get_user_by_id(conn: &DBPool, user_id: Uuid) -> Result<User, DBError> {
    use domain::schema::users::dsl::*;

    users.filter(id.eq(user_id)).first(&mut conn.get().unwrap())
}

pub fn get_user_by_username(conn: &DBPool, username_: &str) -> Result<User, DBError> {
    use domain::schema::users::dsl::*;

    users.filter(username.eq(username_)).first(&mut conn.get().unwrap())
}

pub fn get_user_by_email(conn: &DBPool, email_: &str) -> Result<User, DBError> {
    use domain::schema::users::dsl::*;

    users.filter(email.eq(email_)).first(&mut conn.get().unwrap())
}

pub fn create_user(conn: &DBPool, new_user: NewUser) -> Result<Uuid, DBError> {
    use domain::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .returning(id)
        .get_result(&mut conn.get().unwrap())
}

pub fn update_user(conn: &DBPool, user_id: Uuid, update_user: UpdatedUser) -> Result<(), DBError> {
    use domain::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user_id)))
        .set(&update_user)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn update_user_info(conn: &DBPool, user_id: Uuid, update_user: UpdatedUserInfo) -> Result<(), DBError> {
    use domain::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user_id)))
        .set(&update_user)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_user(conn: &DBPool, user_id: Uuid) -> Result<(), DBError> {
    use domain::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use dotenvy;
    use infrastructure::init_pool;

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

    pub fn test_create_user() -> Uuid {
        let context = TestContext::new();
        let new_user = NewUser {
            username: "test-".to_string() + &Uuid::new_v4().to_string(),
            email: "test-".to_string() + &Uuid::new_v4().to_string()
        };
        let user_id = create_user(&context.conn, new_user).unwrap();
        let updated_user = UpdatedUser {
            username: Some("test-".to_string() + &user_id.to_string()),
            email: Some("test-".to_string() + &user_id.to_string()),
            has_validated_email: None,
            role: None,
            token_version: None,
        };
        update_user(&context.conn, user_id, updated_user).unwrap();
        user_id
    }

    #[test]
    fn test_get_user_by_id() {
        let context = TestContext::new();

        let user_id = test_create_user();

        let user = get_user_by_id(&context.conn, user_id).unwrap();
        assert_eq!(user_id, user.id);
    }

    #[test]
    fn test_get_user_by_username() {
        let context = TestContext::new();

        let user_id = test_create_user();

        let username = format!("test-{}", user_id);
        let user = get_user_by_username(&context.conn, &username).unwrap();
        assert_eq!(user.username, username);
    }

    #[test]
    fn test_get_user_by_email() {
        let context = TestContext::new();

        let user_id = test_create_user();

        let mail = format!("test-{}", user_id);
        let user = get_user_by_email(&context.conn, &mail).unwrap();
        assert_eq!(user.email, mail);
    }

    #[test]
    fn test_update_user() {
        let context = TestContext::new();

        let user_id = test_create_user();

        let random = Uuid::new_v4().to_string();
        let updated_user = UpdatedUser {
            username: None,
            email: Some("test".to_string() + &random),
            has_validated_email: None,
            role: None,
            token_version: None,
        };

        update_user(&context.conn, user_id, updated_user).unwrap();

        let user = get_user_by_id(&context.conn, user_id).unwrap();
        assert_eq!(user.email, "test".to_string() + &random);

        let updated_user_info = UpdatedUserInfo {
            username: Some("test".to_string() + &random),
            email: None,
        };

        update_user_info(&context.conn, user_id, updated_user_info).unwrap();

        let user = get_user_by_id(&context.conn, user_id).unwrap();
        assert_eq!(user.username, "test".to_string() + &random);
    }

    #[test]
    fn test_delete_user() {
        let context = TestContext::new();

        let user_id = test_create_user();
        delete_user(&context.conn, user_id).unwrap();
    }
}