// ############### USER#######
// login - phone, password
// register - fields: name, phone, email, passowrd
// logout
// getProfile - name, phone number, email, favourite addresses
// delete profile

// ########## ADMIN
// delete/add/change_info user - by id
// get user
// change is_blocked field

use actix_web::*;
use crate::resources::db::PostgresPool;
use crate::models::models::{User, Users};
use crate::schemas::schema::users::dsl::*;
use crate::services::user_service::*;
use crate::errors::errors::*;


pub async fn get_users(db: web::Data<PostgresPool>) -> Result<impl Responder, ServiceError> {
    let result =  web::block(move || get_all_users(db)).await;
    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result.unwrap())),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

