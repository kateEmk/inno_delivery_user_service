use std::env;
use actix_web::web::block;
use chrono::{Duration, Utc};
use color_eyre::Result;
use eyre::{eyre, Report};
// use futures::compat::Future01CompatExt;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use password_hash::{PasswordHash, PasswordVerifier, SaltString, rand_core::OsRng, PasswordHasher, };
use scrypt::Scrypt;
use crate::errors::errors::AuthError;


#[derive(Debug, Clone)]
pub struct CryptoService;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

#[derive(Serialize)]
pub struct Auth {
    pub token: String,
}

impl CryptoService {

    pub async fn hash_password_with_salt(password: String) -> password_hash::Result<PasswordHash<'static>>{
        let salt: &'static SaltString  = &SaltString::generate(OsRng);
        return Scrypt.hash_password(password.as_ref(),  salt)
    }

    pub async fn verify_password_with_salt(password: &str, password_hash: &str) -> Result<(), AuthError> {
        let hash = PasswordHash::new(password_hash).map_err(|_| AuthError::VerifyError)?;
        let algs: &[&dyn PasswordVerifier] = &[&Scrypt];
        hash.verify_password( algs, password).map_err(|_| AuthError::VerifyError)
    }

    pub async fn generate_jwt(user_id: i32) -> Result<jsonwebtoken::errors::Result<String>, Report> {
        let jwt_secret_key = get_jwt_secret_key();

        block(move || {
            let headers = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_secret_key.as_bytes());
            let now = Utc::now() + Duration::days(1);
            let claims = Claims {
                sub: user_id,
                exp: now.timestamp(),
            };
            encode(&headers, &claims, &encoding_key)
        })
            .await
        .map_err(|err| eyre!("Creating jwt token: {}", err))
    }

    pub async fn verify_jwt(token: String) -> Result<jsonwebtoken::errors::Result<TokenData<Claims>>, Report> {
        let jwt_secret_key = get_jwt_secret_key();

        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_secret_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation)
        })
        .await
        .map_err(|err| eyre!("Verifying jwt token: {}", err))
    }
}


pub fn get_hash_secret_key() -> String {
    dotenv().ok();
    env::var("hash_secret_key").unwrap_or("none".to_string())
}

pub fn get_jwt_secret_key() -> String {
    dotenv().ok();
    env::var("jwt_secret_key").unwrap_or("none".to_string())
}