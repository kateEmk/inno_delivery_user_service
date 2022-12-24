use std::env;
use dotenv::dotenv;
use argonautica::{Hasher, Verifier};

pub fn hash_password(password: String) -> Result<String, argonautica::Error> {
    dotenv().ok();
    let hash_secret_key = env::var("hash_secret_key").expect("hash_secret_key");

    let mut hasher = Hasher::default();

    hasher
        .with_password(password)
        .with_secret_key(hash_secret_key)
        .hash()
}

pub fn verify_password(hash: String, password: String) -> Result<bool, argonautica::Error> {
    dotenv().ok();
    let hash_secret_key = env::var("hash_secret_key").expect("hash_secret_key");

    let mut verifier = Verifier::default();

    verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(hash_secret_key)
        .verify()
}

#[test]
fn verification_succeeded() {
    let hash = hash_password(String::from("123")).unwrap();
    let hash_verification = verify_password(hash, String::from("123")).unwrap();
    assert_eq!(hash_verification, true);
}

#[test]
fn verification_failed() {
    let hash = hash_password(String::from("123")).unwrap();
    let bad_hash_verification = verify_password(hash, String::from("xnpgu")).unwrap();
    assert_eq!(bad_hash_verification, false);
}