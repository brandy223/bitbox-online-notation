use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{http, web, HttpMessage};
use application::authentication::tokens::decode_token;
use application::database::groups::get_group_by_id;
use application::database::students::get_student_by_id;
use application::database::students_tokens::get_student_token_by_token;
use application::database::tokens::get_token_by_token_string;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use futures_util::FutureExt;
use uuid::Uuid;

use application::database::users::get_user_by_id;
use domain::models::groups::Group;
use domain::models::students::Student;
use domain::models::users::User;
use shared::app_state_model::AppState;
use shared::token_models::{SpecificClaims, StudentClaims, UserClaims};

#[derive(Clone)]
pub struct UserTokenValidator;
impl TokenValidator for UserTokenValidator {
    fn validate(&self, token: &str, app_state: &AppState, req: &ServiceRequest) -> Result<(), actix_web::Error> {
        let config = app_state.config.read().clone();

        // Decode token and handle errors
        let decoded_token = decode_token::<UserClaims>(&token, &config)
            .map_err(|_| ErrorUnauthorized("Error"))?;

        // Check token expiration
        if decoded_token.exp < chrono::Utc::now().timestamp() as usize {
            return Err(ErrorUnauthorized("Token expired"));
        }

        // Check if user exists
        let conn = app_state.database_pool.clone().as_ref().clone();
        let user = get_user_by_id(&conn, decoded_token.sub)
            .map_err(|_| ErrorUnauthorized("Error with user"))?;

        // Check user token version
        if user.token_version != decoded_token.token_version {
            return Err(ErrorUnauthorized("Invalid token"));
        }

        req.extensions_mut().insert::<User>(user);

        Ok(())
    }
}

#[derive(Clone)]
pub struct SpecificTokenValidator;
impl TokenValidator for SpecificTokenValidator {
    fn validate(&self, token: &str, app_state: &AppState, req: &ServiceRequest) -> Result<(), actix_web::Error> {
        let config = app_state.config.read().clone();

        // Decode token and handle errors
        let decoded_token = decode_token::<SpecificClaims>(&token, &config)
            .map_err(|_| ErrorUnauthorized("Error"))?;

        // Check token expiration
        if decoded_token.exp < chrono::Utc::now().timestamp() as usize {
            return Err(ErrorUnauthorized("Token expired"));
        }

        // Get token in the database
        let token = get_token_by_token_string(&app_state.clone().database_pool, token)
            .map_err(|_| ErrorUnauthorized("Error"))?;

        // Check if token type is the same
        if token.type_ != decoded_token.type_ {
            return Err(ErrorUnauthorized("Invalid token"));
        }

        // Check if token is used
        if token.used {
            return Err(ErrorUnauthorized("Token already used"));
        }

        // Check if user exists
        let conn = app_state.database_pool.clone().as_ref().clone();
        let user = get_user_by_id(&conn, decoded_token.sub)
            .map_err(|_| ErrorUnauthorized("Error with user"))?;

        req.extensions_mut().insert::<User>(user);
        req.extensions_mut().insert::<Uuid>(token.id);

        Ok(())
    }
}

#[derive(Clone)]
pub struct StudentTokenValidator;
impl TokenValidator for StudentTokenValidator {
    fn validate(&self, token: &str, app_state: &AppState, req: &ServiceRequest) -> Result<(), actix_web::Error> {
        let config = app_state.config.read().clone();

        // Decode token and handle errors
        let decoded_token = decode_token::<StudentClaims>(&token, &config)
            .map_err(|_| ErrorUnauthorized("Error"))?;

        // Check token expiration
        if decoded_token.exp < chrono::Utc::now().timestamp() as usize {
            return Err(ErrorUnauthorized("Token expired"));
        }

        // Get token in the database
        let token = get_student_token_by_token(&app_state.clone().database_pool, token.to_string())
            .map_err(|_| ErrorUnauthorized("Error"))?;

        // Check if token is used
        if token.used {
            return Err(ErrorUnauthorized("Token already used"));
        }

        // Check if student exists
        let conn = app_state.database_pool.clone().as_ref().clone();
        let student = get_student_by_id(&conn, decoded_token.sub)
            .map_err(|_| ErrorUnauthorized("Error with student"))?;

        let group = get_group_by_id(&conn, decoded_token.group_id)
            .map_err(|_| ErrorUnauthorized("Error with group"))?;

        req.extensions_mut().insert::<Student>(student);
        req.extensions_mut().insert::<Group>(group);
        req.extensions_mut().insert::<Uuid>(token.id);

        Ok(())
    }
}

pub trait TokenValidator {
    fn validate(&self, token: &str, app_state: &AppState, req: &ServiceRequest) -> Result<(), actix_web::Error>;
}

pub struct AuthMiddleware<S, T: TokenValidator> {
    service: Rc<S>,
    validator: T,
}

impl<S, T: Clone> Service<ServiceRequest> for AuthMiddleware<S, T>
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse<BoxBody>,
            Error = actix_web::Error,
        > + 'static,
        T: TokenValidator + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    /// Polls the readiness of the wrapped service.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    /// Handles incoming requests.
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let validator = self.validator.clone();

        // Extract necessary data from req
        let app_state = req.app_data::<web::Data<AppState>>().cloned().unwrap();
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|h| h.to_str().ok())
                    .map(|auth| auth.split_at(7).1.to_string())
            }).or_else(|| {
            req.headers()
                .get("credentials")
                .and_then(|h| h.to_str().ok())
                .map(|cred| cred.to_string())
        });

        Box::pin(async move {
            // If token is missing, return unauthorized error
            let token = token.ok_or_else(|| ErrorUnauthorized("Token not provided"))?;

            // Decode token
            validator.validate(&token, &app_state, &req)?;

            // Call the wrapped service to handle the request
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}

/// Middleware factory for requiring authentication.
pub struct RequireAuth<T: TokenValidator> {
    validator: T,
}

impl<T: TokenValidator> RequireAuth<T> {
    pub fn new(validator: T) -> Self {
        RequireAuth { validator }
    }
}

impl<S, T: Clone> Transform<S, ServiceRequest> for RequireAuth<T>
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse<BoxBody>,
            Error = actix_web::Error,
        > + 'static,
        T: TokenValidator + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S, T>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Creates and returns a new AuthMiddleware wrapped in a Result.
    fn new_transform(&self, service: S) -> Self::Future {
        // Wrap the AuthMiddleware instance in a Result and return it.
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
            validator: self.validator.clone(),
        }))
    }
}