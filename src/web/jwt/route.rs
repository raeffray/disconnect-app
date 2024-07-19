use rocket::{get, http::Status, response::status, serde::json::Json, Response};
use crate::{dao::app_users_dao::find_app_user, db::pool::create_pool, web::jwt::request::{Claims, JwtRequest }};

fn generate_jwt(user_id: &str, secret: &str) -> String {
    use jsonwebtoken::{encode, Header, EncodingKey};
    use chrono::{Utc, Duration};

    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp() as usize;

    let actions = vec!["read".to_string(), "write".to_string()]; // Example actions, replace with actual logic

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        actions: actions
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("valid JWT")
}

#[post("/generate_jwt", format = "application/json", data = "<jwt_request>")]
pub fn generate_jwt_token_route(jwt_request: Json<JwtRequest>) -> Result<status::Custom<String>, status::Custom<String>>  {
    let user_id = &jwt_request.user_id;
    let secret = &jwt_request.secret;

    let pool: std::sync::Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>> = create_pool();

    let result = find_app_user(&pool, &user_id, &secret); 

    match result {
        Ok(user) => {
            let token = generate_jwt(&user.user_id, &user.secret);
            Ok(status::Custom(Status::Ok, token))
        }
        Err(diesel::result::Error::NotFound) => Err(status::Custom(Status::NotFound, "Invalid user ID or secret".to_string())),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Database error".to_string())),
    }
    
}