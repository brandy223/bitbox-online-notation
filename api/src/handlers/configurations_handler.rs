use actix_web::{get, HttpResponse, put, ResponseError, web};
use uuid::Uuid;

use application::database::config::{get_config_by_user_id, update_user_config};
use domain::models::config::UpdatedUserConfig;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, InternalError, ServerError};

use crate::middlewares::auth::RequireAuth;

/// Get user config by user id
///
/// This endpoint returns the configuration of a user (teacher) with the specified id.
#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "Configuration",
    context_path = "/config",
    params(
        ("user_id" = i32, description = "The user id to get the config from")
    ),
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
#[get("/{user_id}")]
pub async fn get_config_by_user_id_route(data: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_config_by_user_id(&conn, id.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(user_config) => HttpResponse::Ok().json(user_config),
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update a user configuration with the user id
///
/// This endpoint updates the configuration of a user (teacher) with the specified user id.
#[utoipa::path(
    put,
    path = "/{user_id}",
    tag = "Configuration",
    context_path = "/config",
    params(
        ("user_id" = Uuid, description = "The promotion of the user to update")
    ),
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
#[put("/{user_id}")]
pub async fn update_user_config_route(data: web::Data<AppState>, user_id: web::Path<Uuid>, updated_config: web::Json<UpdatedUserConfig>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        update_user_config(&conn, user_id.into_inner(), updated_config.into_inner())
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
            .wrap(RequireAuth)
            .service(get_config_by_user_id_route)
            .service(update_user_config_route)
    );
}