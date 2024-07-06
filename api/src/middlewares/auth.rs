use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{http, HttpMessage, web};
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use futures_util::future::{LocalBoxFuture, ready, Ready};
use futures_util::FutureExt;

use application::database::users::get_user_by_id;
use domain::models::users::User;
use shared::app_state_model::AppState;

use crate::handlers::authentication_handler::decode_token;

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse<BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    /// Polls the readiness of the wrapped service.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    /// Handles incoming requests.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Attempt to extract token from cookie or authorization header
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        // If token is missing, return unauthorized error
        if token.is_none() {
            return Box::pin(ready(Err(ErrorUnauthorized("Token not provided"))))
        }

        let app_state = req.app_data::<web::Data<AppState>>().unwrap();

        let config = {
            let config_guard = app_state.config.read();
            config_guard.clone()
        };

        // Decode token and handle errors
        let (user_id, token_version) = match decode_token(
            token.unwrap().as_str(),
            &config,
        ) {
            Ok(id) => id,
            Err(_) => {
                return Box::pin(ready(Err(ErrorUnauthorized("Error"))))
            }
        };

        let cloned_app_state = app_state.clone();
        let srv = Rc::clone(&self.service);

        // Handle user extraction and request processing
        async move {
            // Check if user exists
            let user = match web::block(move || {
                let conn = cloned_app_state.database_pool.clone().as_ref().clone();
                let user = get_user_by_id(&conn, user_id);
                user
            })
                .await
            {
                Ok(user) => match user {
                    Ok(user) => {
                        // Check user token version
                        if user.token_version != token_version {
                            return Err(ErrorUnauthorized("Invalid token"))
                        }
                        user
                    },
                    Err(_) => {
                        return Err(ErrorUnauthorized("User not found"))
                    }
                },
                Err(_) => { return Err(ErrorUnauthorized("Error")) }
            };

            // Insert user information into request extensions
            req.extensions_mut().insert::<User>(user.clone());

            // Call the wrapped service to handle the request
            let res = srv.call(req).await?;
            Ok(res)
        }
            .boxed_local()
    }
}

/// Middleware factory for requiring authentication.
pub struct RequireAuth;

impl<S> Transform<S, ServiceRequest> for RequireAuth
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse<BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Creates and returns a new AuthMiddleware wrapped in a Result.
    fn new_transform(&self, service: S) -> Self::Future {
        // Wrap the AuthMiddleware instance in a Result and return it.
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}