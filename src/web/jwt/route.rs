use rocket::{http::Status, response::status, serde::json::Json};
use crate::{db::pool::create_pool, web::jwt::request::{JwtRequest, JwtResponse}};

use crate::web::jwt::service::generate_jwt;

use super::app_users_dao::{find_app_user, find_system_user};


#[post("/generate_jwt", format = "application/json", data = "<jwt_request>")]
pub fn generate_jwt_token_route(jwt_request: Json<JwtRequest>) -> Result<status::Custom<Json<JwtResponse>>, status::Custom<String>>  {
    let user_id = &jwt_request.user_id;
    let secret = &jwt_request.secret;

    let pool: std::sync::Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>> = create_pool();

    //let result = find_app_user(&pool, &user_id, &secret); 
    //find_system_user(&pool, user_id, secret)
    let result = find_system_user(&pool, user_id, secret);

    let audience = vec!["https://api.example.com".to_string()];
    let issuer = "https://your-auth-server.com";


    match result {
        Ok(pair) => {
            let jwt_response: JwtResponse = generate_jwt(&pair.0.user_id, &pair.0.secret, pair.1, audience, &issuer);
            
            Ok(status::Custom(Status::Ok, Json(jwt_response)))
        }
        Err(diesel::result::Error::NotFound) => Err(status::Custom(Status::NotFound, "Invalid user ID or secret".to_string())),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Database error".to_string())),
    }
    
}