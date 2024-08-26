use actix_web::cookie::{CookieBuilder, SameSite};
use actix_web::{get, web, HttpResponse, ResponseError};
use application::database::students_tokens::get_student_token_by_id;
use application::database::tokens::get_token_by_id;
use domain::models::tokens::TokenType;
use infrastructure::DBPool;
use serde::Deserialize;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, DBError, InternalError, ServerError, UnauthorizedError, UserError};
use uuid::Uuid;

#[derive(Deserialize)]
struct ResetTokenQuery {
    id: Uuid,
}

#[derive(Deserialize)]
struct EvaluationTokenQuery {
    id: Uuid,
}

/// Request password reset token
///
/// This endpoint allows users to request a password reset token which is returned in a cookie.
#[utoipa::path(
    get,
    path = "/reset",
    tag = "Authentication",
    context_path = "/token",
    params(
        ("id" = Uuid, Query, description = "The reset token id")
    ),
    responses(
        (status = 200, description = "Responded with JWT", body = String),
        (status = 401, description = "Unauthorized", body = UnauthorizedError),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/reset")]
async fn get_reset_token_route(
    data: web::Data<AppState>,
    query: web::Query<ResetTokenQuery>,
) -> HttpResponse {
    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        let token_id = query.id;

        // Check if token exists
        let token_object = match get_token_by_id(&conn, token_id) {
            Ok(token) => token,
            Err(err) => return match err {
                DBError::NotFound => Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError))),
                _ => Err(APIError::from(err)),
            }
        };

        // Check if token is valid
        if token_object.type_ != TokenType::PassReset {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }
        if token_object.used {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        Ok(token_object.token)
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

/// Request student token to evaluate his group
///
/// This endpoint allows student to get the token to evaluate his group.
#[utoipa::path(
    get,
    path = "/evaluation",
    tag = "Evaluation",
    context_path = "/token",
    params(
        ("id" = Uuid, Query, description = "The evaluation token id")
    ),
    responses(
        (status = 200, description = "Responded with JWT", body = String),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, example = json!("UnauthorizedError")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/evaluation")]
pub async fn get_evaluation_token_route(
    data: web::Data<AppState>,
    query: web::Query<EvaluationTokenQuery>,
) -> HttpResponse {
    let result = web::block(move || {
        let conn: DBPool = data.database_pool.clone().as_ref().clone();
        let token_id = query.id;

        // Check if token exists
        let token_object = match get_student_token_by_id(&conn, token_id) {
            Ok(token) => token,
            Err(err) => return match err {
                DBError::NotFound => Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError))),
                _ => Err(APIError::from(err)),
            }
        };

        // Check if token is valid
        if token_object.used {
            return Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)));
        }

        Ok(token_object.token)
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

pub fn token_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/token")
            .service(get_reset_token_route)
            .service(get_evaluation_token_route)
    );
}