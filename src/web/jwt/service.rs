use crate::web::jwt::claim::Claims;
use crate::utils::date_util::timestamp_to_readable;

use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use super::request::JwtResponse;

pub(crate) fn generate_jwt(user_id: &str, secret: &str, roles: Vec<String>) -> JwtResponse {

    let expiration: usize = Utc::now()
        .checked_add_signed(Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims: Claims = Claims::builder()
        .sub(user_id.to_owned())
        .exp(expiration)
        .roles(roles)
        .build();

    let readable_expeiration_date = timestamp_to_readable(expiration);

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("valid JWT");

    JwtResponse::builder()
        .expiration(readable_expeiration_date)
        .token(token)
        .build()

}