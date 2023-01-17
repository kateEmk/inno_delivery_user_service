use crate::models::models::{User, CreateNewUser};
use actix_web::web;

use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::middleware::jwt_crypto::CryptoService;
use crate::schema::schema::users::dsl::*;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}


pub async fn signup(mut conn: PooledConnection<ConnectionManager<PgConnection>>, user: web::Json<CreateNewUser>, role_choice: String) -> Result<CreateNewUser, String> {
    let user_already_exists = users
            .filter(email.eq(&user.email))
            .load::<User>(&mut conn)
            .unwrap();

    if user_already_exists.is_empty() {
        let password_hash = CryptoService::hash_password_with_salt(user.password.to_string())
            .await;

        CryptoService::verify_password_with_salt(&*user.password, &password_hash)
            .await
            .expect("Password couldn't be verified");

        let json = json!({
            "first_name": user.first_name,
            "phone_number": user.phone_number,
            "email": user.email,
            "password": String::from_utf8_lossy(&password_hash).to_string(),
            "role": role_choice,
        }).to_owned();
        let converted = json.as_str();

        let user_form = serde_json::from_str::<CreateNewUser>(converted.unwrap()).expect("Couldn't get user");

        diesel::insert_into(users)
            .values(&user_form)
            .execute(&mut conn)
            .expect("Could not create new user");

        return Ok(user_form);
    }
    else {
        return Err(String::from("Email already in use"))
    }
}
