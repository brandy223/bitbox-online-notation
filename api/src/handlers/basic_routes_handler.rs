use actix_web::{get, HttpResponse, Responder};

use crate::models::response_models::GenericResponse;

/// Check if the API is online
///
/// This endpoint returns a simple message to check if the API is online.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "API is online", body = GenericResponse),
    )
)]
#[get("/api/health")]
pub async fn healthcheck() -> impl Responder {
    let response = GenericResponse {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

pub async fn not_found() -> actix_web::Result<HttpResponse> {
    let response = GenericResponse {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}