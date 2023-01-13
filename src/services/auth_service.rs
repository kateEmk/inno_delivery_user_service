use crate::{
    errors::errors::ServiceError,
    models::models::{LoginResponse, Login, User, CreateNewUser, Response},
    middleware::jwt_crypto,
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde::{Serialize, Deserialize};
use crate::middleware::jwt_crypto::CryptoService;
use crate::resources::db::PostgresPool;
use crate::schema::schema::users::dsl::*;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}


pub async fn signup(mut conn: PooledConnection<ConnectionManager<PgConnection>>, user: web::Json<CreateNewUser>, role_choice: String) -> Result<CreateNewUser, Response> {
    let user_already_exists = users
            .filter(email.eq(&user.email))
            .load::<User>(&mut conn)
            .unwrap();
    let res;
    if user_already_exists.is_empty() {
        let password_hash = CryptoService::hash_password_with_salt(user.password.to_string())
            .await
            .expect("Password couldn't be hashed");

        CryptoService::verify_password_with_salt(&*user.password, &*password_hash.to_string())
            .await
            .expect("Password couldn't be verified");

        let new_user = CreateNewUser {
            first_name: (&user.first_name).to_string(),
            phone_number: (&user.phone_number).to_string(),
            email: (&user.email).to_string(),
            password: (password_hash).to_string(),
            role: role_choice
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .expect("Could not create new user");

        res = new_user;
        // return Ok(new_user);
    }
    else {
        res = Response {
            message: "This e-mail is using by some user, please enter another e-mail".to_string(),
            status: false,
        };
    }
    return Ok(res)
    // return Ok(Response {
    //     message: "This e-mail is using by some user, please enter another e-mail.".to_string(),
    //     status: false,
    // })


    // return Err(String::from("Email already in use"))
}
