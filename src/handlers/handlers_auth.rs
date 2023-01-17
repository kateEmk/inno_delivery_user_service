use crate::services::auth_service;
use actix_web::{web, HttpResponse, Responder};
use crate::errors::errors::ServiceError;
use crate::models::models::CreateNewUser;
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
