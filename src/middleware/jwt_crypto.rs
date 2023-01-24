use std::env;
use actix_web::web::block;
use color_eyre::Result;
use eyre::eyre;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use password_hash::{PasswordHash, PasswordVerifier};
use scrypt::Scrypt;
use crate::errors::errors::AuthError;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use std::ops::Deref;

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

    pub fn jwt_factory(claims: Claims) -> String {
        let jwt_secretkey = get_jwt_secret_key();
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secretkey.as_bytes()));
        match token {
            Ok(jwt) => jwt,
            _ => String::from("Could not create token")
        }
    }

    pub async fn hash_password_with_salt(password: String) -> [u8; 64] {
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(100_000).unwrap();
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt).expect("TODO: panic message");

        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );
        return pbkdf2_hash
    }

    pub async fn verify_password_with_salt(password: &str, password_hash: &[u8]) -> Result<(), AuthError> {
        let password_as_string = String::from_utf8_lossy(&password_hash);
        let hash = PasswordHash::new(password_as_string.deref()).map_err(|_| AuthError::VerifyError)?;
        let algs: &[&dyn PasswordVerifier] = &[&Scrypt];
        hash.verify_password(algs, &password.as_bytes()).map_err(|_| AuthError::VerifyError)
    }

    pub async fn verify_jwt(token: String) -> bool {
        let jwt_secret_key = get_jwt_secret_key();

        let decoded_token = block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_secret_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation)
        })
        .await
        .map_err(|err| eyre!("Verifying jwt token: {}", err));

        match decoded_token {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

pub fn get_jwt_secret_key() -> String {
    dotenv().ok();
    env::var("jwt_secret_key").unwrap_or("none".to_string())
}