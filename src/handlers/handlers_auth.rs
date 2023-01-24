use crate::services::auth_service;
use actix_web::{web, HttpResponse, Responder, };
use actix_web::cookie::Cookie;
use crate::errors::errors::{AuthError, ServiceError};
use crate::models::auth_models::{AuthData, UserLoggedIn, UserLoginError};
use crate::models::user_models::{CreateNewUser, RetrieveUserResponse};
use crate::resources::db::PostgresPool;
use tracing::instrument;

#[instrument]
pub async fn register(pool: web::Data<PostgresPool>, item: web::Json<CreateNewUser>) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        auth_service::signup(conn, item)
    }).await.unwrap();

    match user.await {
        Ok(user) => {
            let created_user = RetrieveUserResponse {
                first_name: user.first_name.to_string(),
                phone_number: user.phone_number.to_string(),
                email: user.email.to_string(),
                role: user.role.to_string(),
            };
            Ok(HttpResponse::Created().json(serde_json::to_string(&created_user).unwrap()))
        },
        Err(_err) => {
            // log::error!("{:?}", err);
            Err(ServiceError::BadRequest("Couldn't create user's account".to_string()))
        },
    }
}

#[instrument]
pub async fn login(pool: web::Data<PostgresPool>, auth_data: web::Json<AuthData>) -> impl Responder {
    let conn = pool.get().unwrap();

    let logged_in_user = auth_service::login_user(conn, auth_data.into_inner());

    match logged_in_user.await {
        Ok(user) => {
            let cookie = Cookie::build("refresh_token", user.clone().refresh_token)
                .domain("http://localhost:3000")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(UserLoggedIn {
                    first_name: user.first_name,
                    email: user.email,
                    jwt: user.jwt,
                    refresh_token: user.refresh_token,
                })
        },

        Err(_err) => {
            // log::error!("{:?}", err);
            HttpResponse::Ok().json(UserLoginError {
                message: String::from("Could not log user in"),
                error: AuthError::Unauthorized.to_string(),
            })
        }
    }
}
