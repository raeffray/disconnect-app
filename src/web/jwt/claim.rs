use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder,Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
    pub(crate) roles: Vec<String>,
    pub(crate) aud: Vec<String>,  // Audience can be a single string or a list of strings
    pub(crate) iss: String,       // Issuer of the JWT
}