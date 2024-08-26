use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::tokens::{NewToken, Token, UpdatedToken};
use infrastructure::DBPool;
use uuid::Uuid;

pub fn get_token_by_id(conn: &DBPool, id_: Uuid) -> Result<Token, DBError> {
    use domain::schema::tokens::dsl::*;

    tokens.filter(id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn get_token_by_token_string(conn: &DBPool, token_string: &str) -> Result<Token, DBError> {
    use domain::schema::tokens::dsl::*;

    tokens.filter(token.eq(token_string))
        .first(&mut conn.get().unwrap())
}

pub fn create_token(conn: &DBPool, new_token: NewToken) -> Result<Uuid, DBError> {
    use domain::schema::tokens::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(tokens)
        .values(&new_token)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn update_token(conn: &DBPool, id_: Uuid, updated_token: UpdatedToken) -> Result<(), DBError> {
    use domain::schema::tokens::dsl::*;

    // Check if the token exists
    tokens.filter(id.eq(id_.clone()))
        .first::<Token>(&mut conn.get().unwrap())?;

    diesel::update(tokens.filter(id.eq(id_)))
        .set(&updated_token)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::models::tokens::TokenType;
    use infrastructure::init_pool;
    use uuid::Uuid;

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

    fn test_create_token() -> (Uuid, String) {
        let context = TestContext::new();

        let token = Uuid::new_v4().to_string();
        let new_token = NewToken{
            token: token.clone(),
            type_: TokenType::PassReset,
        };

        (create_token(&context.conn, new_token).unwrap(), token)
    }

    #[test]
    fn test_get_token_by_id() {
        let context = TestContext::new();

        let (token_id, _) = test_create_token();

        let token = get_token_by_id(&context.conn, token_id.clone()).unwrap();

        assert_eq!(token.id, token_id);
    }

    #[test]
    fn test_get_token_by_hashed_token() {
        let context = TestContext::new();

        let (token_id, token_content) = test_create_token();

        let token_by_hashed_token = get_token_by_token_string(&context.conn, &token_content).unwrap();
        assert_eq!(token_by_hashed_token.id, token_id)
    }

    #[test]
    fn test_update_token() {
        let context = TestContext::new();

        let (token_id, _) = test_create_token();

        let updated_token = UpdatedToken{
            type_: Some(TokenType::PassReset),
            used: Some(true),
        };

        update_token(&context.conn, token_id.clone(), updated_token).unwrap();

        let token = get_token_by_id(&context.conn, token_id).unwrap();
        assert_eq!(token.type_, TokenType::PassReset)
    }
}