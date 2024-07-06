use diesel::prelude::*;
use diesel::result::Error as DBError;
use uuid::Uuid;

use domain::models::config::*;
use infrastructure::DBPool;

pub fn get_main_config(conn: &DBPool) -> Result<MainConfig, DBError> {
    use domain::schema::main_config::dsl::*;

    main_config.first(&mut conn.get().unwrap())
}

pub fn get_config_by_user_id(conn: &DBPool, id_: Uuid) -> Result<UserConfig, DBError> {
    use domain::schema::user_config::dsl::*;

    user_config.filter(user_id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn create_main_config(conn: &DBPool, new_main_config: NewMainConfig) -> Result<i32, DBError> {
    use domain::schema::main_config::dsl::*;

    let result: Result<i32, DBError> = diesel::insert_into(main_config)
        .values(&new_main_config)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn create_user_config(conn: &DBPool, new_user_config: NewUserConfig) -> Result<i32, DBError> {
    use domain::schema::user_config::dsl::*;

    let result: Result<i32, DBError> = diesel::insert_into(user_config)
        .values(&new_user_config)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn update_main_config(conn: &DBPool, updated_main_config: UpdatedMainConfig) -> Result<(), DBError> {
    use domain::schema::main_config::dsl::*;

    let id_ = get_main_config(&conn).unwrap().id;

    diesel::update(main_config.filter(id.eq(id_)))
        .set(&updated_main_config)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn update_user_config(conn: &DBPool, id_: Uuid, updated_user_config: UpdatedUserConfig) -> Result<(), DBError> {
    use domain::schema::user_config::dsl::*;

    diesel::update(user_config.filter(user_id.eq(id_)))
        .set(&updated_user_config)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_user_config(conn: &DBPool, id_: i32) -> Result<(), DBError> {
    use domain::schema::user_config::dsl::*;

    diesel::delete(user_config.filter(id.eq(id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

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

    fn test_create_main_config() -> i32 {
        let context = TestContext::new();

        let new_main_config = NewMainConfig{
            register: None,
            authorized_domains: None,
        };

        create_main_config(&context.conn, new_main_config).unwrap()
    }

    fn test_create_user_config() -> (i32, Uuid) {
        let context = TestContext::new();

        let user_id = test_create_user();
        let new_user_config = NewUserConfig{
            user_id,
            alerts: None,
        };

        (create_user_config(&context.conn, new_user_config).unwrap(), user_id)
    }

    fn delete_main_config(id_: i32) -> Result<(), DBError> {
        use domain::schema::main_config::dsl::*;

        let context = TestContext::new();
        diesel::delete(main_config.filter(id.eq(id_)))
            .execute(&mut context.conn.get().unwrap())?;

        Ok(())
    }

    #[test]
    fn test_get_main_config() {
        let context = TestContext::new();

        let _ = test_create_main_config();
        let main_config = get_main_config(&context.conn).unwrap();
        delete_main_config(main_config.id).unwrap();
        assert_eq!(main_config.register, true);
    }

    #[test]
    fn test_get_config_by_user_id() {
        let context = TestContext::new();

        let (_, user_id) = test_create_user_config();
        let user_config = get_config_by_user_id(&context.conn, user_id).unwrap();
        assert_eq!(user_config.user_id, user_id);
    }

    #[test]
    fn test_update_main_config() {
        let context = TestContext::new();

        let _ = test_create_main_config();
        let main_config = get_main_config(&context.conn).unwrap();
        let updated_main_config = UpdatedMainConfig{
            register: Some(true),
            authorized_domains: None,
        };

        update_main_config(&context.conn, updated_main_config).unwrap();
        delete_main_config(main_config.id).unwrap();
    }

    #[test]
    fn test_update_user_config() {
        let context = TestContext::new();

        let (_, user_id) = test_create_user_config();
        let updated_user_config = UpdatedUserConfig{
            alerts: Some(vec![
                serde_json::json!({"type": "success", "message": "Test message"})
            ])
        };

        update_user_config(&context.conn, user_id, updated_user_config).unwrap();
    }

    #[test]
    fn test_delete_user_config() {
        let context = TestContext::new();

        let (id_, _) = test_create_user_config();
        delete_user_config(&context.conn, id_).unwrap()
    }
}