use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
    pub(crate) actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct JwtRequest {
    pub(crate) user_id: String,
    pub(crate) secret: String,
}

