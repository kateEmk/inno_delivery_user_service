use actix_web::{web};
use std::vec::Vec;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;
use crate::models::models::{CreateNewUser, UpdateUserProfile, User};
use crate::schema::schema::users::dsl::*;
use crate::diesel::ExpressionMethods;


pub fn create(conn: &mut PgConnection, user: web::Json<CreateNewUser>) -> Result<CreateNewUser, String> {

    let user_already_exists = users
            .filter(email.eq(&user.email))
            .load::<User>(conn)
            .unwrap();

    if user_already_exists.is_empty() {
        // hash_password(user.password
        //     .to_string())
        //     .expect("Could not hash password");

        let new_user = CreateNewUser {
            id: user.id,
            first_name: (&user.first_name).to_string(),
            phone_number: (&user.phone_number).to_string(),
            email: (&user.email).to_string(),
            password: (&user.password).to_string(),
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .expect("Could not create new user");

        return Ok(new_user);
    }

    Err(String::from("Email already in use"))
}


pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>, Error> {
    let result = users.load::<User>(conn)?;
    Ok(result)
}

pub fn get_user_by_id(conn: &mut PgConnection, id_user: i32) -> Result<User, Error> {
    let user = users
        .filter(id.eq(id_user))
        .first::<User>(conn);
    return user
}

pub fn update(conn: &mut PgConnection, new_user: web::Json<UpdateUserProfile>, id_user: i32) -> Result<(), Error> {

    diesel::update(
        users.filter(id.eq(id_user)))
        .set((
            first_name.eq(&new_user.first_name),
            phone_number.eq(&new_user.phone_number),
            email.eq(&new_user.email),
            password.eq(&new_user.password)
        ))
        .get_result::<User>(conn).unwrap();

    Ok(())
}

pub fn delete(conn: &mut PgConnection, id_user: i32) -> Result<(), Error> {

    diesel::update(users.filter({
            is_deleted.eq(false);
            id.eq(id_user)
        }))
        .set(is_deleted.eq(true))
        .execute(conn)
        .expect("User couldn't be deleted");

    Ok(())
}
