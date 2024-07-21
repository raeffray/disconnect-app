use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use jsonwebtoken::{decode, Validation, DecodingKey, Algorithm};

use super::claim::Claims;

pub struct JwtGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                let decoding_key = DecodingKey::from_secret("wieriwerioweoirwerwerwer".as_ref());
                let validation = Validation::new(Algorithm::HS256);
                match decode::<Claims>(&token, &decoding_key, &validation) {
                    Ok(token_data) => {
                        let claims = token_data.claims;
                        Outcome::Success(JwtGuard { claims })
                    }
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}