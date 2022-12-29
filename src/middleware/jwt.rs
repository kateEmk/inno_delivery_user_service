use std::env;
use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey, decode, Validation, DecodingKey};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
}

pub fn jwt_factory(claims: Claims) -> String {
    dotenv().ok();
    let jwt_secret_key = env::var("jwt_secret_key").expect("jwt_secret_key must be set.");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret_key.as_bytes()));
    match token {
        Ok(jwt) => jwt,
        _ => String::from("Could not create token")
    }
}

pub fn validate_token(token: &str) -> bool {
    dotenv().ok();
    let jwt_secret_key = env::var("jwt_secret_key").expect("jwt_secret_key must be set.");

    let validation = Validation::default();
    let decoding_key = &DecodingKey::from_secret(jwt_secret_key.as_bytes());
    let decoded_access_token = decode::<Claims>(&token, decoding_key, &validation);

    match decoded_access_token {
        Ok(_) => true,
        Err(_) => false
    }
}