use actix_web::web;
use std::vec::Vec;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

extern crate uuid;
use uuid::Uuid;

use crate::models::user_models::{UpdateUserProfile, User};
use crate::schema::schema::users::dsl::*;
use crate::middleware::jwt_crypto::CryptoService;


pub fn get_users(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Result<Vec<User>, Error> {
    let result = users.load::<User>(&mut conn)?;
    return Ok(result)
}

pub async fn get_user_by_id(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid) -> Result<User, Error> {
    users
        .clone()
        .filter(uuid.eq(id_user))
        .first::<User>(&mut conn)
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

    return Ok(())
}

pub async fn update_user_password(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid, new_password: String) -> Result<(), Error> {
    let password_hash = CryptoService::hash_password_with_salt((&new_password).parse().unwrap()).await;

    CryptoService::verify_password_with_salt(&*new_password, &password_hash)
        .await
        .expect("Password couldn't be verified");

    diesel::update(users.filter(uuid.eq(id_user)))
        .set(password.eq(&new_password))
        .execute(&mut conn)
        .expect("User couldn't update password");

    return Ok(())
}

pub fn delete(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid) -> Result<(), Error> {
    diesel::update(users.filter({
            is_deleted.eq(false);
            uuid.eq(id_user)
        }))
        .set(is_deleted.eq(true))
        .execute(&mut conn)
        .expect("User couldn't be deleted");

    return Ok(())
}
