use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Deserialize)]
pub struct JwtRequest {
    pub(crate) user_id: String,
    pub(crate) secret: String,
}

#[derive(TypedBuilder, Debug, Serialize, Deserialize)]
pub struct JwtResponse {
    pub(crate) expiration: String,
    pub(crate) token: String
}