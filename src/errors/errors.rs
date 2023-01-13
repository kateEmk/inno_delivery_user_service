use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorised")]
    Unauthorised,

    #[display(fmt = "User not found")]
    UserNotFound,
}


impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorised => {
                HttpResponse::Unauthorized().json("User is unauthorized")
            }
            ServiceError::UserNotFound => {
                HttpResponse::NotFound().json("User not found")
            }
        }
    }
}

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "Error during validating")]
    ErrorValidating,
    #[display(fmt = "Error during verifying password")]
    VerifyError,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::ErrorValidating => {
                HttpResponse::BadGateway().json("Error during validating (JWT Auth), Please try later")
            },
            AuthError::VerifyError => {
                HttpResponse::ExpectationFailed().json("Error during verifying password.")
            },
        }
    }
}