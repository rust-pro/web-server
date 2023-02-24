use argonautica::{Error as ArgonError, Hasher, Verifier};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, errors::Error as JWTError, Header};
use lazy_static::lazy_static;

use common::Claims;

use crate::app::types::RoleAuth;

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String = std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
    static ref JWT_SECRET_KEY:String = std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

/**
 *  Verify password
 */
pub fn verify_password(hash: &str, password: &str) -> Result<bool, ArgonError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}

/**
 *  Hash password
 */
pub fn generate_hash(password: &str) -> Result<String, ArgonError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
}

pub fn get_jwt_secret_key() -> String {
    JWT_SECRET_KEY.clone()
}

pub fn create_jwt_token(username: String, role: RoleAuth, secret_key: &str) -> Result<String, JWTError> {
    let exp_time = Local::now() + Duration::minutes(60);
    let claims = Claims {
        sub: username,
        exp: exp_time.timestamp(),
        role: role.to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
}


