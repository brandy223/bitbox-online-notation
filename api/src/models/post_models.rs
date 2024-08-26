use application::database::users::{get_user_by_email, get_user_by_username};
use chrono::NaiveDateTime;
use garde::{Error, Validate};
use infrastructure::init_pool;
use once_cell::sync::Lazy as SyncLazy;
use regex::Regex;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterUserPostModel {
    #[garde(email)]
    #[garde(custom(validate_unique_email))]
    pub email: String,
    #[garde(length(min = 3, max = 20))]
    #[garde(pattern(USERNAME_REGEX))]
    #[garde(custom(validate_unique_username))]
    pub username: String,
    #[garde(length(min = 8, max = 64))]
    #[garde(custom(validate_password_policy))]
    pub password: String,
}

struct PasswordContext {
    min_lower_case: usize,
    min_upper_case: usize,
    min_digits: usize,
    min_special_chars: usize
}

static USERNAME_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
    Regex::new(r#"^[a-zA-Z0-9_.-]+$"#).unwrap()
});

fn validate_unique_email(value: &str, _: &()) -> garde::Result {
    let conn = init_pool(dotenvy::var("DATABASE_URL").unwrap().as_str());
    match get_user_by_email(&conn, value) {
        Ok(_) => Err(Error::new("Email already exists")),
        Err(_) => Ok(()),
    }
}

fn validate_unique_username(value: &str, _: &()) -> garde::Result {
    let conn = init_pool(dotenvy::var("DATABASE_URL").unwrap().as_str());
    match get_user_by_username(&conn, value) {
        Ok(_) => Err(Error::new("Username already exists")),
        Err(_) => Ok(()),
    }
}

// TODO: Re-try context implementation
fn validate_password_policy(value: &str, _: &()) -> garde::Result {
    let context = PasswordContext {
        min_lower_case: 1,
        min_upper_case: 1,
        min_digits: 1,
        min_special_chars: 1,
    };
    // Lower case check
    if value.chars().filter(|c| c.is_lowercase()).count() < context.min_lower_case {
        return Err(Error::new("Password must contain at least 1 lower case character"));
    }
    // Upper case check
    if value.chars().filter(|c| c.is_uppercase()).count() < context.min_upper_case {
        return Err(Error::new("Password must contain at least 1 upper case character"));
    }
    // Digit check
    if value.chars().filter(|c| c.is_digit(10)).count() < context.min_digits {
        return Err(Error::new("Password must contain at least 1 digit"));
    }
    // Special character check
    if value.chars().filter(|c| !c.is_alphanumeric()).count() < context.min_special_chars {
        return Err(Error::new("Password must contain at least 1 special character"));
    }
    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginUserPostModel {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequestPostModel {
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordPostModel {
    #[garde(length(min = 8, max = 64))]
    #[garde(custom(validate_password_policy))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewPromotionPostModel {
    #[garde(length(max = 255))]
    #[garde(alphanumeric)]
    pub title: String,
    #[garde(skip)]
    pub start_year: chrono::NaiveDate,
    #[garde(skip)]
    pub end_year: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewProjectPostModel {
    #[garde(length(max = 64))]
    #[garde(alphanumeric)]
    pub name: String,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub start_date: Option<NaiveDateTime>,
    #[garde(skip)]
    pub end_date: NaiveDateTime,
    #[garde(skip)]
    pub notation_period_duration: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewStudentPostModel {
    #[garde(length(max = 64))]
    #[garde(ascii)]
    pub name: String,
    #[garde(length(max = 64))]
    #[garde(ascii)]
    pub surname: String,
    #[garde(length(max = 128))]
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewGroupPostModel {
    #[garde(length(max = 64))]
    #[garde(ascii)]
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewStudentsToGroup {
    pub group_id: Uuid,
    pub students_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GradedStudentPostModel {
    pub student_id: Uuid,
    pub mark: f64,
    pub comment: Option<String>
}