// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "alert_type"))]
    pub struct AlertType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "project_state"))]
    pub struct ProjectState;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type"))]
    pub struct TokenType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AlertType;

    done_alerts (id) {
        id -> Int4,
        #[max_length = 32]
        description -> Nullable<Varchar>,
        project_id -> Uuid,
        #[sql_name = "type"]
        type_ -> AlertType,
        published_at -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        mark -> Nullable<Float8>,
        max_mark -> Int4,
        project_id -> Uuid,
    }
}

diesel::table! {
    groups_students (group_id, student_id) {
        group_id -> Uuid,
        student_id -> Uuid,
        student_mark -> Nullable<Float8>,
        max_mark -> Int4,
    }
}

diesel::table! {
    main_config (id) {
        id -> Int4,
        register -> Bool,
        authorized_domains -> Array<Nullable<Text>>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    marks (project_id, group_id, noted_student_id, grader_student_id) {
        project_id -> Uuid,
        group_id -> Uuid,
        noted_student_id -> Uuid,
        grader_student_id -> Uuid,
        mark -> Float8,
        max_mark -> Int4,
        comment -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProjectState;

    projects (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        start_date -> Timestamp,
        end_date -> Timestamp,
        notation_period_duration -> Int4,
        promotion_id -> Uuid,
        state -> ProjectState,
    }
}

diesel::table! {
    promotions (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        start_year -> Date,
        end_year -> Date,
        teacher_id -> Uuid,
    }
}

diesel::table! {
    promotions_students (promotion_id, student_id) {
        promotion_id -> Uuid,
        student_id -> Uuid,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        surname -> Varchar,
        #[max_length = 128]
        email -> Varchar,
    }
}

diesel::table! {
    students_tokens (id) {
        id -> Uuid,
        token -> Text,
        student_id -> Uuid,
        project_id -> Uuid,
        used -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenType;

    tokens (id) {
        id -> Uuid,
        token -> Text,
        #[sql_name = "type"]
        type_ -> TokenType,
        used -> Bool,
    }
}

diesel::table! {
    user_config (id) {
        id -> Int4,
        user_id -> Uuid,
        alerts -> Array<Nullable<Jsonb>>,
        updated_at -> Timestamp,
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
        has_validated_email -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        role -> UserRole,
        token_version -> Int4,
    }
}

diesel::joinable!(done_alerts -> projects (project_id));
diesel::joinable!(groups -> projects (project_id));
diesel::joinable!(groups_students -> groups (group_id));
diesel::joinable!(groups_students -> students (student_id));
diesel::joinable!(marks -> groups (group_id));
diesel::joinable!(marks -> projects (project_id));
diesel::joinable!(projects -> promotions (promotion_id));
diesel::joinable!(promotions -> users (teacher_id));
diesel::joinable!(promotions_students -> promotions (promotion_id));
diesel::joinable!(promotions_students -> students (student_id));
diesel::joinable!(students_tokens -> projects (project_id));
diesel::joinable!(students_tokens -> students (student_id));
diesel::joinable!(user_config -> users (user_id));
diesel::joinable!(user_passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    done_alerts,
    groups,
    groups_students,
    main_config,
    marks,
    projects,
    promotions,
    promotions_students,
    students,
    students_tokens,
    tokens,
    user_config,
    user_passwords,
    users,
);
