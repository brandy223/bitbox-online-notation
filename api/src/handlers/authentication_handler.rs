use actix_web::{HttpResponse, post, ResponseError, web};
use actix_web::cookie::CookieBuilder;
use chrono::{Duration, Utc};
use garde::Validate;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use uuid::Uuid;

use application::database::config::create_user_config;
use application::database::user_passwords::{create_user_password, get_user_password_by_user_id};
use application::database::users::{create_user, get_user_by_email, get_user_by_username};
use domain::models::config::NewUserConfig;
use domain::models::user_passwords::NewUserPassword;
use domain::models::users::{NewUser, User, UserRole};
use infrastructure::DBPool;
use shared::app_config::Config;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, DBError, InternalError, ServerError, UnauthorizedError, UserError};
use shared::token_models::UserClaims;

use crate::models::post_models::{LoginUserPostModel, RegisterUserPostModel};

struct UserInfo {
    id: Uuid,
    role: UserRole,
    token_version: i32,
}

/// Register a new user
///
/// This endpoint allows users to register a new account.
#[utoipa::path(
    post,
    path = "/register",
    tag = "Authentication",
    context_path = "/auth",
    request_body(
        content = RegisterUserPostModel,
        description = "The credentials of the user to register",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Account created", body = String),
        (status = 400, description = "Bad Request", body = ValidationError),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/register")]
async fn register_route(
    data: web::Data<AppState>,
    info: web::Json<RegisterUserPostModel>,
) -> HttpResponse {
    let result = web::block(move || {
        let config = {
            let config_guard = data.config.read();
            config_guard.clone()
        };

        if config.clone().main_config.register == false {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        // Check if email domain in credentials is part of the domains' whitelist
        let email_domain = info.email.split('@').collect::<Vec<&str>>()[1];
        if config.clone().main_config.authorized_domains.len() != 0 &&
            !config.clone().main_config.authorized_domains.contains(&Some(email_domain.to_string())) {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        let conn = data.database_pool.clone().as_ref().clone();
        let credentials = info.into_inner();

        credentials.validate()?;

        // Hash password
        let hashed_password = match bcrypt::hash(&credentials.password, bcrypt::DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => return Err(APIError::ServerError(ServerError::InternalError(InternalError))),
        };

        // Insert user into database
        let user_id = create_user(&conn, NewUser {
            email: credentials.email.to_string(),
            username: credentials.username.to_string(),
        })?;

        // Insert password into database
        create_user_password(&conn, NewUserPassword {
            user_id,
            password: hashed_password,
        })?;

        // Init user config
        create_user_config(&conn, NewUserConfig{
            user_id,
            alerts: None,
        })?;

        let info = UserInfo {
            id: user_id,
            role: UserRole::User,
            token_version: 0,
        };

        let token = encode_token(&info, &config)?;
        Ok(token)
    }).await;

    match result {
        Ok(user) => match user {
            Ok(token) => {
                let cookie = CookieBuilder::new("jwt", token)
                    .http_only(true)
                    .secure(true)
                    .finish();
                HttpResponse::Created().cookie(cookie).finish()
            },
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

/// Login a user
///
/// This endpoint allows users to log in to their account.
#[utoipa::path(
    post,
    path = "/login",
    tag = "Authentication",
    context_path = "/auth",
    request_body(
        content = LoginUserPostModel,
        description = "The credentials of the user to login",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, example = json!("Unauthorized")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/login")]
async fn login_route(
    data: web::Data<AppState>,
    info: web::Json<LoginUserPostModel>,
) -> HttpResponse {
    let result = web::block(move || {
        let config = {
            let config_guard = data.config.read();
            config_guard.clone()
        };

        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        let credentials = info.into_inner();

        // Check user
        let user = match get_user_from_body(&conn, &credentials.login) {
            Ok(user) => user,
            Err(err) => return match err {
                DBError::NotFound => Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError))),
                _ => Err(APIError::from(err)),
            },
        };
        let info = UserInfo {
            id: user.id,
            role: user.role,
            token_version: user.token_version,
        };

        let user_password = match get_user_password_by_user_id(&conn, user.id) {
            Ok(user_password) => user_password,
            Err(err) => return match err {
                DBError::NotFound => Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError))),
                _ => Err(APIError::from(err)),
            },
        };
        if bcrypt::verify(&credentials.password, &user_password.password)? {
            let token = encode_token(&info, &config)?;
            Ok(token)
        } else {
            Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)))
        }
    }).await;

    match result {
        Ok(user) => match user {
            Ok(token) => {
                let cookie = CookieBuilder::new("jwt", token)
                    .http_only(true)
                    .secure(true)
                    .finish();
                HttpResponse::Ok().cookie(cookie).finish()
            },
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

fn get_user_from_body(conn: &DBPool, login: &str) -> Result<User, DBError> {
    let user = if login.contains('@') {
        // Check if email exists
        match get_user_by_email(&conn, login) {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    } else {
        // Check if username exists
        match get_user_by_username(&conn, login) {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    };
    user
}

pub fn encode_token(info: &UserInfo, app_config: &Config) -> Result<String, APIError> {
    let now = Utc::now();
    let expiration = now + Duration::hours(app_config.jwt_config.expires_in.parse::<i64>().unwrap());

    let claims = UserClaims {
        sub: info.id,
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
        token_version: info.token_version,
        user_role: info.role,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(app_config.jwt_config.secret.as_bytes()))
        .map_err(|_| APIError::ServerError(ServerError::InternalError(InternalError)))
}

pub fn decode_token(token: &str, app_config: &Config) -> Result<Uuid, APIError> {
    let validation = Validation::new(Algorithm::HS512);
    let token_data = decode::<UserClaims>(token, &DecodingKey::from_secret(app_config.jwt_config.secret.as_bytes()), &validation)
        .map_err(|_| APIError::UserError(UserError::Unauthorized(UnauthorizedError)))?;

    Ok(token_data.claims.sub)
}

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register_route)
            .service(login_route)
    );
}