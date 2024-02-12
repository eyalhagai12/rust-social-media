use std::env;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::database::models::users::User;

pub fn encode_user(user: &User) -> String {
    let secret = env::var("JWT_SECRET_KEY").expect("JWT secret must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).expect("failed to create JWT key");

    user.sign_with_key(&key).expect("failed to sign JWT").to_string()
}

pub fn decode_user(token: String) -> User {
    let secret = env::var("JWT_SECRET_KEY").expect("JWT secret must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).expect("failed to create JWT key");

    token.verify_with_key(&key).expect("failed to decode user JWT")
}