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

    #[display(fmt = "DB not found")]
    DBNotFound,

    #[display(fmt = "User not found")]
    UserNotFound,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
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
            ServiceError::DBNotFound => {
                HttpResponse::NotFound().json("Database not found")
            }
            ServiceError::UserNotFound => {
                HttpResponse::NotFound().json("User not found")
            }
        }
    }
}