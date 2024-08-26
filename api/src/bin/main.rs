use actix_cors::Cors;
use actix_settings::{ApplySettings, Mode, Settings};
use actix_web::middleware::{Compress, Condition, Logger};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use parking_lot::lock_api::Mutex;
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::handlers::basic_routes_handler::*;
use api::{docs::swagger_config::ApiDoc, handlers};
use application::database::config::{create_main_config, get_main_config};
use application::database::user_passwords::create_user_password;
use application::database::users::{create_user, get_user_by_username, update_user};
use application::mail::init::init_smtp_client;
use application::scheduler::init::init_projects_check;
use domain::models::config::NewMainConfig;
use domain::models::user_passwords::NewUserPassword;
use domain::models::users::{NewUser, UpdatedUser, UserRole};
use infrastructure::{init_pool, DBPool};
use shared::app_state_model::AppState;
use shared::error_models::DBError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Init the environment
    let app_state = init_app_state();
    check_admin(&app_state.database_pool);
    let mut settings = Settings::parse_toml("api/src/Server.toml")
        .expect("Failed to parse `Settings` from Server.toml");

    // If the environment variable `$APPLICATION__HOSTS` is set,
    // have its value override the `settings.actix.hosts` setting:
    Settings::override_field_with_env_var(&mut settings.actix.hosts, "APPLICATION__HOSTS")?;

    init_logger(&settings);
    let projects_checker = init_projects_check(&app_state);
    // Modify `projects_checker` in `runtime_values` to store the interval ID in the `AppState`
    app_state.runtime_values.write().projects_checker = Arc::new(Mutex::new(Some(projects_checker)));

    // Create the OpenAPI documentation0
    let openapi = ApiDoc::openapi();
    let content = serde_json::to_string_pretty(&openapi)?;
    std::fs::write("docs/openapi.json", content).expect("Unable to write documentation use api::handlers::marks_handler::get_evaluation_token_route;file");

    // Start the server
    HttpServer::new({
        let settings = settings.clone();
        move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .wrap(Condition::new(
                    settings.actix.enable_compression,
                    Compress::default(),
                ))
                // make `Settings` available to handlers
                .app_data(Data::new(settings.clone()))
                // enable logger
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                // make `AppState` available to handlers
                .app_data(app_state.clone())
                // set up the on-demand documentation
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
                )
                // provision the API routes
                .service(healthcheck)
                .service(
                    web::scope("/api")
                        .configure(handlers::authentication_handler::auth_config)
                        .configure(handlers::promotions_handler::promotions_config)
                        .configure(handlers::configurations_handler::configurations_config)
                        .configure(handlers::admin_handler::admin_config)
                        .configure(handlers::projects_handler::projects_config)
                        .configure(handlers::students_handler::students_config)
                        .configure(handlers::groups_handler::groups_config)
                        .configure(handlers::marks_handler::marks_config)
                        .configure(handlers::tokens_handler::token_config)
                )
                .default_service(web::route().to(not_found))
        }
    })
        // apply the `Settings` to Actix Web's `HttpServer`
        .try_apply_settings(&settings)?
        // .bind_openssl(("127.0.0.1", 8443), ())?
        .run()
        .await
}

/// Initialize the logging infrastructure.
fn init_logger(settings: &Settings) {
    if !settings.actix.enable_log {
        return;
    }

    env::set_var(
        "RUST_LOG",
        match settings.actix.mode {
            Mode::Development => "actix_web=debug",
            Mode::Production => "actix_web=info",
        },
    );

    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
}

// Initialize the application state
fn init_app_state() -> Data<AppState> {
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = init_pool(&database_url);
    let main_config = match get_main_config(&database_pool) {
        Ok(config) => config,
        Err(err) => match err {
            DBError::NotFound => {
                create_main_config(&database_pool, NewMainConfig {
                    register: None,
                    authorized_domains: None,
                }).unwrap();
                get_main_config(&database_pool).unwrap()
            }
            _ => panic!("Error when creating main config")
        }
    };
    let app_config = shared::app_config::Config::init(main_config);
    let smtp_transport = init_smtp_client(&app_config.clone());
    Data::new(AppState::init(database_pool, smtp_transport, app_config))
}

// Check if the admin account exists, if not create it
fn check_admin(conn: &DBPool) {
    match get_user_by_username(conn, "admin") {
        Ok(_) => return,
        Err(err) => match err {
            DBError::NotFound => {
                // Create admin account
                let email = match env::var("ADMIN_EMAIL") {
                    Ok(email) => {
                        if email.is_empty() {
                            "".to_string()
                        } else { email }
                    }
                    Err(_) => "".to_string()
                };
                let user_id = create_user(conn, NewUser {
                    username: "admin".to_string(),
                    email: email.clone(),
                }).unwrap();
                let updated_user = UpdatedUser {
                    username: None,
                    email: None,
                    // If the email is provided, the email is automatically validated
                    has_validated_email: if Some(email) == None { None } else { Some(true) },
                    role: Some(UserRole::Admin),
                    token_version: None,
                };
                update_user(conn, user_id.clone(), updated_user).unwrap();

                // Create password
                let password = match env::var("ADMIN_PASSWORD") {
                    Ok(password) => {
                        if password.is_empty() {
                            env::var("DEFAULT_ADMIN_PASSWORD").unwrap()
                        } else { password }
                    }
                    Err(_) => env::var("DEFAULT_ADMIN_PASSWORD").unwrap()
                };
                let hashed_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
                    Ok(h) => h,
                    Err(_) => panic!("Error when hashing admin password")
                };

                create_user_password(conn, NewUserPassword {
                    user_id: user_id.clone(),
                    password: hashed_password,
                }).unwrap();
            }
            _ => panic!("Error when checking admin account")
        }
    }
}