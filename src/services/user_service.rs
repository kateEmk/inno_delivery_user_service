use actix_web::web;
use std::vec::Vec;
use bcrypt::{hash, DEFAULT_COST};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

extern crate uuid;
use uuid::Uuid;

use crate::models::user_models::{UpdateUserProfile, User};
use crate::schema::schema::users::dsl::*;


pub fn get_users(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Result<Vec<User>, Error> {
    let result = users.load::<User>(&mut conn)?;
    return Ok(result)
}

pub async fn get_user_by_id(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid) -> Result<User, Error> {
    Ok(users
        .clone()
        .filter(uuid.eq(id_user))
        .get_result::<User>(&mut conn).unwrap())
}

pub fn update(mut conn: PooledConnection<ConnectionManager<PgConnection>>, new_user: web::Json<UpdateUserProfile>, id_user: Uuid) -> Result<User, Error>  {
    Ok(diesel::update(
        users.filter(uuid.eq(id_user)))
        .set((
            first_name.eq(&new_user.first_name),
            phone_number.eq(&new_user.phone_number),
            email.eq(&new_user.email),
            password.eq(&new_user.password)
        ))
        .get_result::<User>(&mut conn).unwrap())
}

pub async fn update_user_password(mut conn: PooledConnection<ConnectionManager<PgConnection>>, id_user: Uuid, new_password: String) -> Result<(), Error> {
    let password_hash = hash(new_password, DEFAULT_COST);

    diesel::update(users.filter(uuid.eq(id_user)))
        .set(password.eq(&password_hash.unwrap()))
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
