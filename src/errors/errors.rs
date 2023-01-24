use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

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
            ServiceError::UserNotFound => {
                HttpResponse::NotFound().json("User not found")
            }
        }
    }
}

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "Error during verifying password")]
    VerifyError,
    #[display(fmt = "User doesn't authorised")]
    Unauthorized
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::VerifyError => {
                HttpResponse::ExpectationFailed().json("Error during verifying password.")
            },
            AuthError::Unauthorized => {
                HttpResponse::Unauthorized().json("User doesn't authorised.")
            }
        }
    }
}