use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use jsonwebtoken::{decode, Validation, DecodingKey, Algorithm, errors::{ErrorKind, Error}};
use std::env;

use super::claim::Claims;


pub struct JwtGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let secret_key = env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
                let mut validation = Validation::new(Algorithm::HS256);
                validation.validate_exp = true; // Explicitly ensure that the expiration is checked
                validation.leeway = 0;

                println!("Validation settings: {:?}", validation);
                match decode::<Claims>(&token, &decoding_key, &validation) {
                    Ok(token_data) => {
                        println!("Token decoded successfully: {:?}", token_data.claims);
                        println!("Token expiration time: {:?}", token_data.claims.exp);                
                        Outcome::Success(JwtGuard { claims: token_data.claims })
                    },
                    Err(err) => match *err.kind() {
                        ErrorKind::ExpiredSignature => {
                            eprintln!("JWT token has expired: {:?}", err);
                            Outcome::Error((Status::Unauthorized, ()))
                        },
                        _ => {
                            eprintln!("JWT decode error: {:?}", err);
                            Outcome::Error((Status::Unauthorized, ()))
                        }
                    }
                }
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
