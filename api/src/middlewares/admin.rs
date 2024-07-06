use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{Error, HttpMessage};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorForbidden;
use futures_util::future::{LocalBoxFuture, ready, Ready};

use domain::models::users::{User, UserRole};

pub struct AdminMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AdminMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_admin = req.extensions()
            .get::<User>()
            .map(|user| user.role == UserRole::Admin)
            .unwrap_or(false);

        let fut = self.service.call(req);
        Box::pin(async move {
            if is_admin {
                fut.await
            } else {
                Err(ErrorForbidden("Forbidden"))
            }
        })
    }
}

pub struct RequireAdminRole;

impl<S, B> Transform<S, ServiceRequest> for RequireAdminRole
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AdminMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddleware {
            service: Rc::new(service),
        }))
    }
}