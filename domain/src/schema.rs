// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type"))]
    pub struct TokenType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    promotions (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        start_year -> Date,
        end_year -> Date,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenType;

    tokens (id) {
        id -> Text,
        #[sql_name = "type"]
        type_ -> TokenType,
        used -> Bool,
    }
}

diesel::table! {
    user_passwords (user_id) {
        user_id -> Uuid,
        #[max_length = 255]
        password -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        role -> UserRole,
        token_version -> Int4,
    }
}

diesel::joinable!(user_passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    promotions,
    tokens,
    user_passwords,
    users,
);
