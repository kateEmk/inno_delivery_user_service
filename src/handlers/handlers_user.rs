use actix_web::*;

extern crate uuid;
use uuid::Uuid;

use crate::resources::db::{PostgresPool};
use crate::errors::errors::*;
use crate::models::models::{CreateNewUser, UpdateUserProfile};
use crate::services::user_service::*;


pub async fn create_user(pool: web::Data<PostgresPool>, item: web::Json<CreateNewUser>) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        create(conn, item)
    }).await;

    match user {
        Ok(user) => Ok(HttpResponse::Created().json(())),
        Err(_err) => Err(ServiceError::Unauthorised),
    }
}

pub async fn get_user(pool: web::Data<PostgresPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        get_user_by_id(conn, *user_id)
    }).await;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user.unwrap())),
        Err(_e) => Err(ServiceError::UserNotFound),
    }
}

pub async fn get_all_users(pool: web::Data<PostgresPool>) -> impl Responder {
    let conn = pool.get().unwrap();

    let result =  web::block(move || {
        get_users(conn)
    }).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result.unwrap())),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn update_user(pool: web::Data<PostgresPool>, user_profile: web::Json<UpdateUserProfile>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let result = web::block(move || {
        update(conn, user_profile, *user_id)
    })
        .await
        .expect("User couldn't be updated");

    match result {
        Ok(_result) => Ok(HttpResponse::Ok().body("User updated")),
        Err(_e) => Err(ServiceError::BadRequest("User couldn't be updated".parse().unwrap())),
    }
}

// get name / address / phone_number / email
// change password / address / phone_number / email
//

pub async fn delete_user(pool: web::Data<PostgresPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let result = web::block(move || {
        delete(conn, *user_id)
    })
        .await
        .expect("User couldn't be deleted");

    match result {
        Ok(_result) => Ok(HttpResponse::Accepted().body("User deleted")),
        Err(_e) => Err(ServiceError::UserNotFound),
    }
}
