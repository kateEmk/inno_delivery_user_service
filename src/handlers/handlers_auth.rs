use crate::services::auth_service;
use actix_web::{web, HttpResponse, Responder, };
use actix_web::cookie::Cookie;
use uuid::Uuid;
use crate::errors::errors::{AuthError, ServiceError};
use crate::models::auth_models::{AuthData, UserLoginError, UserLoginResponse};
use crate::models::user_models::CreateNewUser;
use crate::resources::db::PostgresPool;


pub async fn register(pool: web::Data<PostgresPool>, item: web::Json<CreateNewUser>, role: String) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        auth_service::signup(conn, item, role)
    }).await;

    match user {
        Ok(ref _user) => Ok(HttpResponse::Created().json("User created")),
        Err(_err) => Err(ServiceError::Unauthorised),
    }
}

pub async fn login(pool: web::Data<PostgresPool>, auth_data: web::Json<AuthData>, id: Uuid) -> impl Responder {
    let conn = pool.get().unwrap();

    let logged_in_user = auth_service::login_user(conn, auth_data.into_inner());

    match logged_in_user.await {
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

        Err(error) => {
            HttpResponse::Ok().json(UserLoginError {
                message: String::from("Could not log user in"),
                error: AuthError::Unauthorized.to_string(),
            })
        }
    }
}
