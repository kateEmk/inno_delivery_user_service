use actix_web::*;

extern crate uuid;
use uuid::Uuid;

use crate::resources::db::{PostgresPool};
use crate::errors::errors::*;
use crate::services::courier_service::*;

#[get("/")]
pub async fn get_all_couriers(pool: web::Data<PostgresPool>) -> impl Responder {
    let conn = pool.get().unwrap();

    let result =  web::block(move || {
        get_couriers(conn)
    }).await.unwrap();

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(_err) => {
            Err(ServiceError::InternalServerError)
        },
    }
}

#[get("/{uuid}")]
pub async fn get_courier_rating(pool: web::Data<PostgresPool>, courier_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let rating = web::block(move || {
        get_rating(conn, *courier_id)
    }).await;

    match rating.unwrap() {
        Ok(rating) => Ok(HttpResponse::Ok().json(rating.as_slice())),
        Err(_err) => {
            Err(ServiceError::UserNotFound)
        },
    }
}

// WILL BE IMPLEMENTED AFTER ORDER SERVICE USING gRPC
// pub async fn update_courier_rating(pool: web::Data<PostgresPool>, user_id: web::Path<Uuid>, new_rating: f64)  { }