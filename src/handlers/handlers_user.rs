// ############### USER#######
// login - phone, password
// register - fields: name, phone, email, passowrd
// getProfile - name, phone number, email, favourite addresses

// ########## ADMIN
// delete/add/change_info user - by id
// get user
// change is_blocked field

use std::result;
use actix_web::*;
use crate::resources::db::{PostgresPool};
use crate::schema::schema::users::dsl::*;
use crate::errors::errors::*;
use crate::models::models::{CreateNewUser, UpdateUserProfile};
use crate::services::user_service::*;


pub async fn create_user(pool: web::Data<PostgresPool>, item: web::Json<CreateNewUser>) -> impl Responder {
    let user = web::block(move || {
        let mut conn = pool.get().unwrap();
        create(&mut conn, item)
    }).await;

    match user {
        Ok(user) => Ok(HttpResponse::Created().json(user.unwrap())),
        Err(_err) => Err(ServiceError::Unauthorised)
    }
}


pub async fn get_user(pool: web::Data<PostgresPool>, user_id: web::Path<i32>) -> impl Responder {
    let user = web::block(move || {
        let mut conn = pool.get().unwrap();
        get_user_by_id(&mut conn, *user_id)
    }).await;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user.unwrap())),
        Err(_e) => Err(ServiceError::UserNotFound),
    }
}

pub async fn get_all_users(pool: web::Data<PostgresPool>) -> impl Responder {
    let result =  web::block(move || {
        let mut conn = pool.get().unwrap();
        get_users(&mut conn)
    }).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result.unwrap())),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn update_user(pool: web::Data<PostgresPool>, user_profile: web::Json<UpdateUserProfile>, user_id: web::Path<i32>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        update(&mut conn, user_profile, *user_id)
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

pub async fn delete_user(pool: web::Data<PostgresPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        delete(&mut conn, *user_id)
    })
        .await
        .expect("User couldn't be deleted");

    match result {
        Ok(_result) => Ok(HttpResponse::Accepted().body("User deleted")),
        Err(_e) => Err(ServiceError::UserNotFound),
    }
}
