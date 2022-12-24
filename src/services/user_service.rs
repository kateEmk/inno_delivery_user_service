use crate::diesel::RunQueryDsl;
use actix_web::web;
use std::vec::Vec;
use crate::resources::db::PostgresPool;
use crate::models::models::{User};
use crate::schemas::schema::users::dsl::*;


pub fn get_all_users(pool: web::Data<PostgresPool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let result = users.load::<User>(&mut conn)?;
    Ok(result)
}

