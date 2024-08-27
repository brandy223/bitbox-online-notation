use crate::middlewares::auth::{RequireAuth, SpecificTokenValidator};
use crate::models::post_models::{LoginUserPostModel, RegisterUserPostModel, ResetPasswordPostModel, ResetPasswordRequestPostModel, ValidateMFACodePostModel};
use actix_web::cookie::{CookieBuilder, SameSite};
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use application::authentication::codes::generate_random_code;
use application::authentication::password_reset::request_password_reset;
use application::authentication::tokens::encode_token;
use application::database::config::create_user_config;
use application::database::mfa_codes::{create_mfa_code, get_mfa_code_by_id};
use application::database::tokens::update_token;
use application::database::user_passwords::{create_user_password, get_user_password_by_user_id, update_user_password};
use application::database::users::{create_user, get_user_by_email, get_user_by_id, get_user_by_username};
use application::mail::send::{build_mail, send_mail, MailProps};
use chrono::{Duration, Utc};
use domain::models::config::{Alert, NewUserConfig};
use domain::models::mfa_codes::NewMfaCode;
use domain::models::tokens::UpdatedToken;
use domain::models::user_passwords::{NewUserPassword, UpdatedUserPassword};
use domain::models::users::{NewUser, User};
use garde::Validate;
use infrastructure::DBPool;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, DBError, InternalError, ServerError, UnauthorizedError, UserError};
use shared::token_models::UserClaims;
use uuid::Uuid;

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
            alerts: Some(vec![
                Alert {
                    before_event: false,
                    hours: 24,
                },
                Alert {
                    before_event: true,
                    hours: 24,
                },
            ]),
        })?;

        let now = Utc::now();
        let expiration = now + Duration::hours(config.jwt_config.expires_in.parse::<i64>().unwrap());
        let claim = UserClaims{
            sub: user_id,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
            token_version: 0,
        };
        let token = encode_token::<UserClaims>(&claim, &config)?;
        Ok(token)
    }).await;

    match result {
        Ok(user) => match user {
            Ok(token) => {
                let cookie = CookieBuilder::new("token", token)
                    .http_only(false)
                    .secure(false)
                    .same_site(SameSite::Strict)
                    .path("/")
                    .finish();
                HttpResponse::Created().cookie(cookie).finish()
            },
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

/// Login a user and send a code
///
/// This endpoint allows users to init log in to their account and send a code to authenticate.
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
        (status = 200, description = "1st step of login successful", body = Uuid),
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

        let user_password = match get_user_password_by_user_id(&conn, user.id) {
            Ok(user_password) => user_password,
            Err(err) => return match err {
                DBError::NotFound => Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError))),
                _ => Err(APIError::from(err)),
            },
        };

        // Generate MFA Code
        let code = generate_random_code(6);
        let new_mfa_code = NewMfaCode {
            user_id: user.id,
            code: code.clone(),
            exp: None,
        };
        let mfa_code_id = create_mfa_code(&conn, new_mfa_code)?;

        // Send email
        let mail = build_mail(MailProps {
            from: "Bitbox <no-reply@sigma-bot.fr>".to_string(),
            to: user.email.clone(),
            subject: "Authentication Code".to_string(),
            body: format!(
                "Here's the code to enter to authenticate : {}",
                code
            ),
        });
        send_mail(&data.smtp_transport.as_ref(), mail)?;

        if bcrypt::verify(&credentials.password, &user_password.password)? {
            Ok(mfa_code_id)
        } else {
            Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)))
        }
    }).await;

    // match result {
    //     Ok(user) => match user {
    //         Ok(token) => {
    //             let cookie = CookieBuilder::new("token", token)
    //                 .http_only(false)
    //                 .secure(false)
    //                 .same_site(SameSite::Strict)
    //                 .path("/")
    //                 .finish();
    //             HttpResponse::Ok().cookie(cookie).finish()
    //         },
    //         Err(err) => err.error_response(),
    //     },
    //     Err(_) => ServerError::InternalError(InternalError).error_response()
    // }
    match result {
        Ok(id) => match id {
            Ok(id) => HttpResponse::Ok().json(id),
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

/// Validate MFA Code
///
/// This endpoint allows users to validate their MFA code and finally log in into their account.
#[utoipa::path(
    post,
    path = "/login/code/{id}",
    tag = "Authentication",
    context_path = "/auth",
    params(
        ("id" = Uuid, description = "The MFA code id to authenticate")
    ),
    request_body(
        content = ValidateMFACodePostModel,
        description = "The MFA code to log in",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 400, description = "Bad Request", body = ValidationError),
        (status = 401, description = "Unauthorized", body = UnauthorizedError),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/login/code/{id}")]
async fn validate_mfa_code_route(
    data: web::Data<AppState>,
    mfa_code_id: web::Path<Uuid>,
    given_code: web::Json<ValidateMFACodePostModel>,
) -> HttpResponse {
    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        let config = {
            let config_guard = data.config.read();
            config_guard.clone()
        };

        let mfa_code_id = mfa_code_id.into_inner();

        let mfa_code = get_mfa_code_by_id(&conn, mfa_code_id)?;

        // Check if code is correct
        if mfa_code.code != given_code.code {
            println!("2");
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        // Check if code is expired
        if mfa_code.exp < Utc::now().naive_utc() - Duration::hours(2) {
            println!("Exp : {}", mfa_code.exp);
            println!("{}", Utc::now().naive_utc() - Duration::hours(2));
            println!("1");
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        // Get user
        let user = get_user_by_id(&conn, mfa_code.user_id)?;
        println!("3");

        // Generate token
        let now = Utc::now();
        let expiration = now + Duration::hours(config.jwt_config.expires_in.parse::<i64>().unwrap());
        let claim = UserClaims{
            sub: user.id,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
            token_version: user.token_version,
        };
        println!("4");
        let token = encode_token::<UserClaims>(&claim, &config)?;

        Ok(token)
    }).await;
    match result {
        Ok(user) => match user {
            Ok(token) => {
                let cookie = CookieBuilder::new("token", token)
                    .http_only(false)
                    .secure(false)
                    .same_site(SameSite::Strict)
                    .path("/")
                    .finish();
                HttpResponse::Ok().cookie(cookie).finish()
            },
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

/// Request a password reset
///
/// This endpoint allows users to request a password reset.
#[utoipa::path(
    post,
    path = "/reset-request",
    tag = "Authentication",
    context_path = "/auth",
    request_body(
        content = ResetPasswordRequestPostModel,
        description = "The email of the user to reset the password",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Password reset request sent", body = String),
        (status = 400, description = "Bad Request", body = ValidationError),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/reset-request")]
async fn request_reset_password_route(
    data: web::Data<AppState>,
    info: web::Json<ResetPasswordRequestPostModel>,
) -> HttpResponse {
    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        let value = info.into_inner();
        value.validate()?;
        let email = value.email;

        // Check if email exists
        let user = match get_user_by_email(&conn, &email) {
            Ok(user) => Some(user),
            Err(err) => match err {
                DBError::NotFound => None,
                _ => return Err(APIError::from(err)),
            },
        };
        if let Some(user) = user {
            tokio::spawn( async move {
                request_password_reset(&data, user);
            });
        }

        Ok(())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => err.error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response()
    }
}

/// Reset user's password
///
/// This endpoint allows users to reset their password with the given token.
#[utoipa::path(
    post,
    path = "/password",
    tag = "Authentication",
    context_path = "/reset",
    request_body(
        content = ResetPasswordPostModel,
        description = "The new password of the user",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Password reset", body = String),
        (status = 400, description = "Bad Request", body = ValidationError),
        (status = 401, description = "Unauthorized", body = UnauthorizedError),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[post("/reset/password")]
async fn reset_password_route(
    req: HttpRequest,
    data: web::Data<AppState>,
    info: web::Json<ResetPasswordPostModel>,
) -> HttpResponse {
    let user = req.extensions().get::<User>().cloned();
    let token_id = req.extensions().get::<Uuid>().cloned();
    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        let credentials = info.into_inner();

        credentials.validate()?;

        println!("{:?}", credentials);

        // Hash password
        let hashed_password = match bcrypt::hash(&credentials.password, bcrypt::DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => return Err(APIError::ServerError(ServerError::InternalError(InternalError))),
        };

        println!("{:?}", hashed_password);

        // Update password
        if let Some(user) = user {
            update_user_password(&conn, user.id, UpdatedUserPassword {
                password: Some(hashed_password),
            })?;
        } else {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        println!("{:?}", token_id);

        // Update token
        update_token(&conn, token_id.unwrap(), UpdatedToken{
            type_: None,
            used: Some(true),
        })?;

        println!("{:?}", token_id);

        Ok(())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
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

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register_route)
            .service(login_route)
            .service(validate_mfa_code_route)
            .service(request_reset_password_route)
    );
}

pub fn password_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reset")
            .wrap(RequireAuth::new(SpecificTokenValidator))
            .service(reset_password_route)
    );
}