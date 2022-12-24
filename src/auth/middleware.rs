use actix_web::{HttpResponse, Responder, web};
use actix_web::cookie::Cookie;
use serde::{Serialize, Deserialize};
use crate::auth::utils::add_user;
use crate::errors::errors::ServiceError;
use crate::models::models::CreateNewUser;
use crate::resources::db::PostgresPool;


pub async fn login(item: web::Json<CreateNewUser>, db: web::Data<PostgresPool>) -> impl Responder {
    let user = web::block(move || add_user(db, item)).await;
    match user {
        // Ok(result) => Ok(HttpResponse::Created().json(result.unwrap())),
        // Err(_e) => Err(ServiceError::Unauthorised)
        Ok(user) => {

            let cookie = Cookie::build("refresh_token", user.clone().refresh_token.unwrap())
                .domain("http://localhost:3000")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(UserLoginResponse {
                    user_logged_in: user
                })
        },

        Err(error) => Err(ServiceError::Unauthorised)
    }
}

// pub async fn logout(item: web::Json<>)
