use domain::models::tokens::{NewToken, Token, UpdatedToken};
use infrastructure::DBPool;
use diesel::result::Error as DBError;
use diesel::prelude::*;

pub fn get_token_by_id(conn: &DBPool, _id: String) -> Result<Token, DBError> {
    use domain::schema::tokens::dsl::*;

    tokens.filter(id.eq(_id))
        .first(&mut conn.get().unwrap())
}

pub fn create_token(conn: &DBPool, new_token: NewToken) -> Result<String, DBError> {
    use domain::schema::tokens::dsl::*;

    let result: Result<String, DBError> = diesel::insert_into(tokens)
        .values(&new_token)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn update_token(conn: &DBPool, _id: String, updated_token: UpdatedToken) -> Result<(), DBError> {
    use domain::schema::tokens::dsl::*;

    // Check if the token exists
    tokens.filter(id.eq(_id.clone()))
        .first::<Token>(&mut conn.get().unwrap())?;

    diesel::update(tokens.filter(id.eq(_id)))
        .set(&updated_token)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use infrastructure::init_pool;
    use uuid::Uuid;
    use domain::models::tokens::TokenType;
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

    fn test_create_token() -> String {
        let context = TestContext::new();

        let new_token = NewToken{
            id: Uuid::new_v4().to_string(),
            type_: TokenType::PassReset,
        };

        create_token(&context.conn, new_token).unwrap()
    }

    #[test]
    fn test_get_token_by_id() {
        let context = TestContext::new();

        let token_id = test_create_token();

        let token = get_token_by_id(&context.conn, token_id.clone()).unwrap();

        assert_eq!(token.id, token_id);
    }

    #[test]
    fn test_update_token() {
        let context = TestContext::new();

        let token_id = test_create_token();

        let updated_token = UpdatedToken{
            type_: Some(TokenType::StudentMarks),
            used: Some(true),
        };

        update_token(&context.conn, token_id.clone(), updated_token).unwrap();

        let token = get_token_by_id(&context.conn, token_id).unwrap();
        assert_eq!(token.type_, TokenType::StudentMarks)
    }
}