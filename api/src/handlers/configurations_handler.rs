use actix_web::{get, put, web, HttpMessage, HttpRequest, HttpResponse, ResponseError};

use application::database::config::{get_config_by_user_id, update_user_config};
use domain::models::config::UpdatedUserConfig;
use domain::models::users::User;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, DBError, InternalError, NotFoundError, ServerError, UnauthorizedError, UserError};

use crate::middlewares::auth::{RequireAuth, UserTokenValidator};

/// Get current user config
///
/// This endpoint returns the current user (teacher) configuration.
#[utoipa::path(
    get,
    path = "/user",
    tag = "Configuration",
    context_path = "/config",
    responses(
        (status = 200, description = "The current user configuration", body = UserConfig),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("Database record")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/user")]
pub async fn get_config_by_user_id_route(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let user = req.extensions().get::<User>().cloned();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        if let Some(user) = user {
            match get_config_by_user_id(&conn, user.id) {
                Ok(user_config) => Ok(user_config),
                Err(err) => match err {
                    DBError::NotFound => Err(APIError::UserError(UserError::NotFound(NotFoundError {
                        resource: "User configuration not found".to_string(),
                    }))),
                    _ => Err(APIError::from(err)),
                },
            }
        } else {
            Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)))
        }
    }).await;

    match result {
        Ok(response) => match response {
            Ok(user_config) => HttpResponse::Ok().json(user_config),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update current user configuration
///
/// This endpoint updates the configuration of the current user (teacher).
#[utoipa::path(
    put,
    path = "/user",
    tag = "Configuration",
    context_path = "/config",
    request_body(
        content = UpdatedUserConfig,
        description = "The updated user configuration object",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The user configuration updated successfully"),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
            ("NoToken" = (value = json!("Token not provided"))),
            ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("Database record")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[put("/user")]
pub async fn update_user_config_route(req: HttpRequest, data: web::Data<AppState>, updated_config: web::Json<UpdatedUserConfig>) -> HttpResponse {
    let user = req.extensions().get::<User>().cloned();
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        if let Some(user) = user {
            update_user_config(&conn, user.id, updated_config.into_inner()).map_err(|err | APIError::from(err))
        } else {
            Err(APIError::UserError(UserError::Unauthorized(UnauthorizedError)))
        }
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

pub fn configurations_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/config")
            .wrap(RequireAuth::new(UserTokenValidator))
            .service(get_config_by_user_id_route)
            .service(update_user_config_route)
    );
}