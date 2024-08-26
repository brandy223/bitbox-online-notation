use std::fmt::Debug;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
pub use diesel::result::Error as DBError;
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Error, ToSchema, ToResponse, Display)]
pub struct ValidationError {
    field: String,
}

#[derive(Debug, Error, ToSchema, ToResponse, Display)]
pub struct NotFoundError {
    pub resource: String,
}

#[derive(Debug, Error, ToSchema, ToResponse, Display)]
pub struct UnauthorizedError;

#[derive(Debug, Error, ToSchema, ToResponse, Display)]
pub struct ForbiddenError;

#[derive(Debug, Error, ToSchema, ToResponse, Display)]
pub struct BadRequestError {
    request: String,
}

#[derive(Debug, Error, ToSchema, ToResponse, Display)]
pub struct InternalError;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "Validation error : {}", ValidationError.field)]
    ValidationError (ValidationError),
    #[display(fmt = "Resource not found: {}", NotFoundError.resource)]
    NotFound (NotFoundError),
    #[display(fmt = "Unauthorized")]
    Unauthorized (UnauthorizedError),
    #[display(fmt = "Forbidden")]
    Forbidden (ForbiddenError),
    #[display(fmt = "Bad request: {}", BadRequestError.request)]
    BadRequest (BadRequestError),
}

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError (InternalError),
}

#[derive(Debug, Display, Error)]
pub enum APIError {
    UserError(UserError),
    ServerError(ServerError),
    DBError(DBError),
}

impl From<serde_json::Error> for APIError {
    fn from(_error: serde_json::Error) -> Self {
        APIError::ServerError(ServerError::InternalError(InternalError))
    }
}

impl From<DBError> for APIError {
    fn from(err: DBError) -> Self {
        match err {
            DBError::NotFound => APIError::UserError(UserError::NotFound(NotFoundError {
                resource: "Database record".to_string(),
            })),
            _ => APIError::ServerError(ServerError::InternalError(InternalError)),
        }
    }
}

impl From<garde::Error> for APIError {
    fn from(err: garde::Error) -> Self {
        APIError::UserError(UserError::ValidationError(ValidationError {
            field: err.message().to_string()
        }))
    }
}

impl From<garde::Report> for APIError {
    fn from(err: garde::Report) -> Self {
        APIError::UserError(UserError::ValidationError(ValidationError {
            field: err.into_inner().iter().map(|(_, e)| e.message().to_string()).collect::<Vec<String>>().join(", ")
        }))
    }
}

impl From<bcrypt::BcryptError> for APIError {
    fn from(_error: bcrypt::BcryptError) -> Self {
        APIError::UserError(UserError::Unauthorized(UnauthorizedError))
    }
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::NotFound { .. } => StatusCode::NOT_FOUND,
            UserError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            UserError::Forbidden { .. } => StatusCode::FORBIDDEN,
            UserError::BadRequest { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

impl error::ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

impl error::ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            APIError::UserError(user_error) => user_error.status_code(),
            APIError::ServerError(server_error) => server_error.status_code(),
            APIError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            APIError::UserError(user_error) => user_error.error_response(),
            APIError::ServerError(server_error) => server_error.error_response(),
            _ => { HttpResponse::InternalServerError().finish() }
        }
    }
}