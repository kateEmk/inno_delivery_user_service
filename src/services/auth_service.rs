use crate::models::user_models::{User, CreateNewUser, RetrieveUserResponse};
use actix_web::web;
use chrono::{Duration, Utc};

use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use crate::errors::errors::AuthError;
use crate::middleware::jwt_crypto::{Claims, CryptoService};
use crate::models::auth_models::{AuthData, UserLoggedIn};
use crate::models::courier_models::{Couriers};
use crate::schema::schema::courier::dsl::courier;
use crate::schema::schema::users::dsl::*;
use bcrypt::{DEFAULT_COST, verify, hash};



pub async fn signup(mut conn: PooledConnection<ConnectionManager<PgConnection>>, user: web::Json<CreateNewUser>) -> Result<RetrieveUserResponse, String> {
    let result = diesel::select(exists(
        users.filter(email.eq(&user.email))))
        .get_result(&mut conn);
    assert_eq!(result, Ok(false));

    let password_hash = hash(&user.password, DEFAULT_COST);

    println!("{}", user.email);
    let new_user = User {
        first_name: (user.first_name).to_string(),
        address: String::default(),
        phone_number: (user.phone_number).to_string(),
        email: (user.email).to_string(),
        password: password_hash.unwrap(),
        role: (user.role).to_string(),
        is_blocked: false,
        is_deleted: false,
        created_at: Default::default(),
        updated_at: Default::default(),
        uuid: ::uuid::Uuid::new_v4().to_string().parse().unwrap(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Could not create new user");

    if new_user.role == "Courier" {
        let new_courier = Couriers {
            is_free: true,
            rating: 5.0,
            uuid: new_user.uuid,
        };

        if user.role == "Courier" {
            diesel::insert_into(courier)
                .values(&new_courier)
                .execute(&mut conn)
                .expect("Could not add new courier");
        }
    }

    let form = RetrieveUserResponse{
        first_name: new_user.first_name,
        phone_number: new_user.phone_number,
        email: new_user.email,
        role: new_user.role,
    };
    return Ok(form)
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
    let valid = verify(user_to_login.password, existing_user.password.as_str());
    assert_eq!(valid.unwrap(), true);

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
    CryptoService::verify_jwt(String::from(&refresh_jwt)).await;

    let logged_in_user = UserLoggedIn {
        first_name: existing_user.first_name,
        email: existing_user.email,
        jwt: CryptoService::jwt_factory(access_token_claims),
        refresh_token: refresh_jwt
    };
    Ok(logged_in_user)
}