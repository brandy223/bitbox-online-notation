use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use dotenvy::dotenv;
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use api::{handlers, docs::swagger_config::ApiDoc};
use api::handlers::basic_routes_handler::*;

use shared::app_state_model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");

    dotenv().ok();

    let app_config = shared::app_config::Config::init();
    let app_state = web::Data::new(AppState::init(app_config));

    env_logger::try_init_from_env(Env::default().default_filter_or("info")).unwrap();

    let openapi = ApiDoc::openapi();
    let content = serde_json::to_string_pretty(&openapi).unwrap();
    std::fs::write("docs/openapi.json", content).expect("Unable to write documentation file");

    println!("Starting server at: http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(healthcheck)
            .service(
                web::scope("/api")
                    .configure(handlers::authentication_handler::auth_config)
                    .configure(handlers::promotions_handler::promotions_config)
            )
            .default_service(web::route().to(not_found))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}