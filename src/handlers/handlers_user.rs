use actix_web::*;

extern crate uuid;
use uuid::Uuid;

use crate::resources::db::{PostgresPool};
use crate::errors::errors::*;
use crate::models::models::UpdateUserProfile;
use crate::services::user_service::*;
use std::result::Result;
use diesel::result::Error;


pub async fn get_user(pool: web::Data<PostgresPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();

    let user = web::block(move || {
        get_user_by_id(conn, *user_id)
    }).await;

    match user {
        Ok(ref user) => Ok(HttpResponse::Ok().json("User {user} created")),
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

    let user_updated = web::block(move || {
        update(conn, user_profile, *user_id)
    })
        .await
        .expect("User couldn't be updated");

    match user_updated {
        Ok(ref user_updated) => Ok(HttpResponse::Ok().body("User {user_updated} has been updated")),
        Err(_e) => Err(ServiceError::BadRequest("User couldn't be updated".parse().unwrap())),
    }
}

pub async fn update_field(pool: web::Data<PostgresPool>, id_user: Uuid, field: String, new_value: String) -> impl Responder {
    let conn = pool.get().unwrap();

    let user_updated = web::block(move || {
        update_exact_field(conn, id_user, field, new_value)
    })
        .await
        .expect("User's field couldn't be updated");

    match user_updated.await {
        Ok(ref user_updated) => Ok(HttpResponse::Ok().body("User {user_updated} has been updated")),
        Err(_e) => Err(ServiceError::BadRequest("User couldn't be updated".parse().unwrap())),
    }
}

pub async fn update_password(pool: web::Data<PostgresPool>, id_user: Uuid, new_password: String) -> impl Responder {
    let conn = pool.get().unwrap();

    let result = web::block(move || {
        update_user_password(conn, id_user, new_password)
    })
        .await
        .expect("User's field couldn't be updated");

    match result.await {
        Ok(_result) => Ok(HttpResponse::Ok().body("User;s password has been updated")),
        Err(_e) => Err(ServiceError::BadRequest("User couldn't be updated".parse().unwrap())),
    }
}

pub async fn get_field(pool: web::Data<PostgresPool>, id_user: Uuid, field: Option<String>) -> Result<Option<String>, Error> {
    let conn = pool.get().unwrap();

    web::block(move || {
        extract_field(conn, id_user, field)
    })
        .await
        .expect("Field couldn't be extracted")

}


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
