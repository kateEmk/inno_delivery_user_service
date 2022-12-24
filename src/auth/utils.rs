use actix_web::error::JsonPayloadError::Serialize;
use actix_web::web;
use diesel::expression_methods::ExpressionMethods;
use diesel::{insert_into, query_dsl::QueryDsl};
use diesel::result::Error;
use crate::diesel::RunQueryDsl;
use crate::errors::errors::ServiceError;
use crate::models::models::{CreateNewUser, User};
use crate::resources::db::PostgresPool;
use crate::schemas::schema::users::dsl::*;
use crate::schemas::schema::users::*;

pub fn add_user(pool: web::Data<PostgresPool>, item: web::Json<CreateNewUser>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();

    let new_user = CreateNewUser {
        id: item.id,
        first_name: (&item.first_name).to_string(),
        phone_number: (&item.phone_number).to_string(),
        email: (&item.email).to_string(),
        password: (&item.password).to_string(),
    };

    insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)
        .expect("Error adding person");

    let mut items = users
        .filter(id.eq(&item.id))
        .load::<User>(&mut conn)
        .expect("Error adding person");

    Ok(items)
}


pub fn delete_user(pool: web::Data<PostgresPool>, user_id: i32) -> usize {
    let mut conn = pool.get().unwrap();

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&mut conn)
        .expect("Error deleting user")
}