use actix_web::*;

extern crate uuid;
use uuid::Uuid;

use crate::resources::db::{PostgresPool};
use crate::errors::errors::*;
use crate::models::user_models::{RetrieveUserResponse, UpdateUserProfile};
use crate::services::user_service::*;
use tracing::instrument;

#[instrument(skip(user_id), fields(uuid = %user_id))]
pub async fn get_user(pool: web::Data<PostgresPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        get_user_by_id(conn, *user_id)
    }).await.unwrap();
    match user.await {
        Ok(user) => {
            let user_info = RetrieveUserResponse {
                first_name: user.first_name.to_string(),
                phone_number: user.phone_number.to_string(),
                email: user.email.to_string(),
                role: user.role.to_string(),
            };
            Ok(HttpResponse::Ok().json(serde_json::to_string(&user_info).unwrap()))
        },
        Err(_err) => {
            // log::error!("{:?}", err);
            Err(ServiceError::UserNotFound) },
    }
}

#[instrument]
pub async fn get_all_users(pool: web::Data<PostgresPool>) -> impl Responder {
    let conn = pool.get().unwrap();

    let result =  web::block(move || {
        get_users(conn)
    }).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result.unwrap())),
        Err(_err) => {
            // log::error!("{:?}", err);
            Err(ServiceError::InternalServerError) },
    }
}

#[instrument]
pub async fn update_user(pool: web::Data<PostgresPool>, user_profile: web::Json<UpdateUserProfile>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let user_updated = web::block(move || {
        update(conn, user_profile, *user_id)
    }).await;

    match user_updated.unwrap() {
        Ok(user_updated) => {
            let user_info = RetrieveUserResponse {
                first_name: user_updated.first_name.to_string(),
                phone_number: user_updated.phone_number.to_string(),
                email: user_updated.email.to_string(),
                role: user_updated.role.to_string(),
            };
            Ok(HttpResponse::Ok().json(serde_json::to_string(&user_info).unwrap()))
        },
        Err(_err) => {
            // log::error!("{:?}", err);
            Err(ServiceError::BadRequest("User couldn't be updated".parse().unwrap())) },
    }
}

#[instrument]
pub async fn update_password(pool: web::Data<PostgresPool>, id_user: web::Path<Uuid>, new_password: String) -> impl Responder {
    let conn = pool.get().unwrap();

    let result = web::block(move || {
        update_user_password(conn, *id_user, new_password)
    }).await.expect("User's field couldn't be updated");

    match result.await {
        Ok(_result) => Ok(HttpResponse::Ok().body("User's password has been updated")),
        Err(_err) => {
            // log::error!("{:?}", err);
            Err(ServiceError::BadRequest("User couldn't be updated".parse().unwrap())) },
    }
}

#[instrument]
pub async fn delete_user(pool: web::Data<PostgresPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let result = web::block(move || {
        delete(conn, *user_id)
    })
        .await
        .expect("User couldn't be deleted");

    match result {
        Ok(_result) => Ok(HttpResponse::Accepted().body("User has been deleted")),
        Err(_err) => {
            // log::error!("{:?}", err);
            Err(ServiceError::UserNotFound) },
    }
}
