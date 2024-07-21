use std::env;

use crate::web::jwt::claim::Claims;
use crate::utils::date_util::timestamp_to_readable;

use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use super::request::JwtResponse;

pub(crate) fn generate_jwt(user_id: &str, secret: &str, roles: Vec<String>, audience: Vec<String>, issuer: &str) -> JwtResponse {
    
    let secret: String = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set");

    let exp_str: String = env::var("JWT_EXPIRATION_SECONDS").expect("Expiration time not set");
    let exp_seconds: i64 = exp_str.parse().expect("Invalid number for expiration seconds");

    let expiration: usize = Utc::now()
        .checked_add_signed(Duration::seconds(exp_seconds))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims: Claims = Claims::builder()
        .sub(user_id.to_owned())
        .exp(expiration)
        .roles(roles)
        .aud(audience)
        .iss(issuer.to_owned())
        .build();

    let readable_expiration_date = timestamp_to_readable(expiration);

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("valid JWT");

    JwtResponse::builder()
        .expiration(readable_expiration_date)
        .token(token)
        .build()
}
