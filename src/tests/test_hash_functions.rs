// use crate::middleware::hash::{hash_password, verify_password};
//
// #[test]
// fn verification_succeeded() {
//     let hash = hash_password(String::from("123")).unwrap();
//     let hash_verification = verify_password(hash, String::from("123")).unwrap();
//     assert_eq!(hash_verification, true);
// }
//
// #[test]
// fn verification_failed() {
//     let hash = hash_password(String::from("123")).unwrap();
//     let bad_hash_verification = verify_password(hash, String::from("xnpgu")).unwrap();
//     assert_eq!(bad_hash_verification, false);
// }