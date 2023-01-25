use std::env;
use actix_web::web::block;
use eyre::eyre;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use ring::digest;
use ring::error::Unspecified;
use pbkdf2::*;

extern crate base64;


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