use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};
use utoipauto::utoipauto;

#[derive(Debug, Serialize)]
struct Security;

impl Modify for Security {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "api_key",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[utoipauto(paths =
    "api/src, ./shared/src from shared, ./domain/src/models from domain, ./application/src from application"
)]
#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "/api")
    ),
    info(
        title = "BitBox API",
        version = "0.1.0",
        description = "Rust API for 360 Project notation online",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    modifiers(&Security),
    security(
        ("api_key" = ["read", "write"])
    )
)]
pub struct ApiDoc;