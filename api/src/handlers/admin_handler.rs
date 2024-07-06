use actix_web::{get, HttpResponse, put, ResponseError, web};

use application::database::config::{get_main_config, update_main_config};
use domain::models::config::UpdatedMainConfig;
use shared::app_state_model::AppState;
use shared::error_models::{APIError, InternalError, ServerError};

use crate::middlewares::admin::RequireAdminRole;
use crate::middlewares::auth::RequireAuth;

/// Get main app configuration
///
/// This endpoint returns the main configuration of Bitbox.
#[utoipa::path(
    get,
    path = "/config/",
    tag = "Admin",
    context_path = "/admin",
    responses(
        (status = 200, description = "The main config of the application", body = MainConfig),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
        ("NoToken" = (value = json!("Token not provided"))),
        ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 403, description = "Forbidden", body = String, example = json!("Forbidden")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[get("/config/")]
pub async fn get_main_config_route(data: web::Data<AppState>) -> HttpResponse {
    let result = web::block(move || {
        let conn = data.database_pool.clone().as_ref().clone();
        get_main_config(&conn)
    }).await;

    match result {
        Ok(response) => match response {
            Ok(config) => HttpResponse::Ok().json(config),
            Err(err) => APIError::from(err).error_response()
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

/// Update the main configuration
///
/// This endpoint updates the main configuration of the application.
#[utoipa::path(
    put,
    path = "/config/",
    tag = "Admin",
    context_path = "/admin",
    request_body(
        content = UpdatedMainConfig,
        description = "The updated main configuration object",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Main configuration updated successfully"),
        (status = 401, description = "Unauthorized", body = UnauthorizedError, examples(
        ("NoToken" = (value = json!("Token not provided"))),
        ("InvalidToken" = (value = json!("Error")))
        )),
        (status = 403, description = "Forbidden", body = String, example = json!("Forbidden")),
        (status = 404, description = "Not Found", body = NotFoundError, example = json!("Database record")),
        (status = 500, description = "Internal Server Error", body = InternalError, example = json!("InternalError")),
    )
)]
#[put("/config/")]
pub async fn update_main_config_route(data: web::Data<AppState>, updated_config: web::Json<UpdatedMainConfig>) -> HttpResponse {
    let conn = data.database_pool.clone().as_ref().clone();
    let result = web::block(move || {
        update_main_config(&conn, updated_config.into_inner())
    }).await;

    match result {
        Ok(response) => match response {
            Ok(_) => {
                let config_update = get_main_config(&data.database_pool.clone().as_ref().clone());
                return match config_update {
                    Ok(main_config) => {
                        let mut config = data.config.write();
                        config.main_config = main_config.clone();
                        HttpResponse::Ok().finish()
                    },
                    Err(_) => ServerError::InternalError(InternalError).error_response(),
                }
            },
            Err(err) => APIError::from(err).error_response(),
        },
        Err(_) => ServerError::InternalError(InternalError).error_response(),
    }
}

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(RequireAdminRole)
            .wrap(RequireAuth)
            .service(get_main_config_route)
            .service(update_main_config_route)
    );
}