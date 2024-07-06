use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenvy::dotenv;
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use api::{docs::swagger_config::ApiDoc, handlers};
use api::handlers::basic_routes_handler::*;
use application::database::config::{create_main_config, get_main_config};
use application::mail::init::init_smtp_client;
use domain::models::config::NewMainConfig;
use infrastructure::init_pool;
use shared::app_state_model::AppState;
use shared::error_models::DBError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");

    dotenv().ok();
    let app_state = init_app_state();

    // TODO : Need to create admin user if not exists for app init

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
                    .configure(handlers::configurations_handler::configurations_config)
            )
            .default_service(web::route().to(not_found))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn init_app_state() -> Data<AppState> {
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = init_pool(&database_url);
    let main_config = match get_main_config(&database_pool) {
        Ok(config) => config,
        Err(err) => match err{
            DBError::NotFound => {
                create_main_config(&database_pool, NewMainConfig {
                    register: None,
                    authorized_domains: None,
                }).unwrap();
                get_main_config(&database_pool).unwrap()
            },
            _ => panic!("Error when creating main config")
        }
    };
    let app_config = shared::app_config::Config::init(main_config);
    let smtp_transport = init_smtp_client(&app_config.clone());
    Data::new(AppState::init(database_pool, smtp_transport, app_config))
}