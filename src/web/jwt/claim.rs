use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder,Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
    pub(crate) roles: Vec<String>,
}