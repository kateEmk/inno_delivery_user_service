use crate::models::user_models::{User, CreateNewUser, UserProfile};
use actix_web::web;
use chrono::{Duration, Utc};

use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde_json::json;
use crate::errors::errors::AuthError;
use crate::middleware::jwt_crypto::{Claims, CryptoService};
use crate::models::auth_models::{AuthData, UserLoggedIn};
use crate::schema::schema::users::dsl::*;


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

pub async fn login_user(mut conn: PooledConnection<ConnectionManager<PgConnection>>, auth_data: AuthData) -> Result<UserLoggedIn, AuthError> {
    let existing_user = users
            .filter(email.eq(&auth_data.email))
            .get_result::<User>(&mut conn);

    match existing_user {
        Ok(registered_user) => handle_login(registered_user, auth_data).await,
        Err(_e) => Err(AuthError::Unauthorized),
    }
}


pub async fn handle_login(existing_user: User, user_to_login: AuthData) -> Result<UserLoggedIn, AuthError> {
    let password_hash = CryptoService::hash_password_with_salt((&user_to_login.password).parse().unwrap()).await;
    CryptoService::verify_password_with_salt(&*existing_user.password, &password_hash).await.expect("Password couldn't be verified");


    let fifteen_min_from_now = Utc::now() + Duration::minutes(15);
    let timestamp_for_access = usize::try_from(fifteen_min_from_now.timestamp()).unwrap();
    let access_token_claims = Claims {
        sub: existing_user.uuid.get_version_num() as i32,
        exp: timestamp_for_access as i64
    };

    let one_week_from_now = Utc::now() + Duration::days(7);
    let timestamp_for_refresh = usize::try_from(one_week_from_now.timestamp()).unwrap();
    let refresh_token_claims = Claims {
        sub: existing_user.uuid.get_version_num() as i32,
        exp: timestamp_for_refresh as i64,
    };

    let refresh_jwt = CryptoService::jwt_factory(refresh_token_claims);
    CryptoService::verify_jwt(String::from(&refresh_jwt));

    let logged_in_user = UserLoggedIn {
        first_name: existing_user.first_name,
        email: existing_user.email,
        jwt: CryptoService::jwt_factory(access_token_claims),
        refresh_token: Option::from(refresh_jwt),
    };
    Ok(logged_in_user)

}