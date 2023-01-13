use crate::services::auth_service;
use actix_web::{web, HttpRequest, HttpResponse, Result, Responder};
use crate::errors::errors::ServiceError;
use crate::models::models::CreateNewUser;
use crate::resources::db::PostgresPool;


// POST api/auth/signup
pub async fn register(pool: web::Data<PostgresPool>, item: web::Json<CreateNewUser>, role: String) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        auth_service::signup(conn, item, role)
    }).await;

    match user {
        Ok(ref user) => Ok(HttpResponse::Created().json("User {user} created")),
        Err(_err) => Err(ServiceError::Unauthorised),
    }
}
