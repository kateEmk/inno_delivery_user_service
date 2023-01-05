use actix_web::{web};
use std::vec::Vec;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use r2d2::PooledConnection;

extern crate uuid;
use uuid::Uuid;

use crate::models::models::{CreateNewUser, UpdateUserProfile, User};
use crate::schema::schema::users::dsl::*;
use crate::diesel::ExpressionMethods;
use crate::middleware::jwt::CryptoService;


pub async fn create(mut conn: PooledConnection<ConnectionManager<PgConnection>>, user: web::Json<CreateNewUser>) -> Result<CreateNewUser, String> {
    let user_already_exists = users
            .filter(email.eq(&user.email))
            .load::<User>(&mut conn)
            .unwrap();

    if user_already_exists.is_empty() {
        let password_hash = CryptoService::hash_password(user.password.to_string())
            .await
            .expect("Password couldn't be hashed");

        CryptoService::verify_password(&*user.password, &*password_hash)
            .await
            .expect("Password couldn't be verified");

        let new_user = CreateNewUser {
            first_name: (&user.first_name).to_string(),
            phone_number: (&user.phone_number).to_string(),
            email: (&user.email).to_string(),
            password: (&user.password).to_string(),
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .expect("Could not create new user");

        return Ok(new_user);
    }

    Err(String::from("Email already in use"))
}


pub fn get_users(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Result<Vec<User>, Error> {
    let result = users.load::<User>(&mut conn)?;
    Ok(result)
}

pub fn get_user_by_id(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid) -> Result<User, Error> {
    let user = users
        .filter(uuid.eq(id_user))
        .first::<User>(&mut conn);
    return user
}

pub fn update(mut conn: PooledConnection<ConnectionManager<PgConnection>>, new_user: web::Json<UpdateUserProfile>, id_user: Uuid) -> Result<(), Error> {

    diesel::update(
        users.filter(uuid.eq(id_user)))
        .set((
            first_name.eq(&new_user.first_name),
            phone_number.eq(&new_user.phone_number),
            email.eq(&new_user.email),
            password.eq(&new_user.password)
        ))
        .get_result::<User>(&mut conn).unwrap();

    Ok(())
}

pub fn delete(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid) -> Result<(), Error> {

    diesel::update(users.filter({
            is_deleted.eq(false);
            uuid.eq(id_user)
        }))
        .set(is_deleted.eq(true))
        .execute(&mut conn)
        .expect("User couldn't be deleted");

    Ok(())
}
