use std::str::FromStr;

use rocket::{get, http::Status, response::status, serde::json::Json};

use role_guard_macro::log_execution;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{dao::fellow_dao::{create_fellow, find_fellow}, db::pool::create_pool, domain::{fellow::{Fellow, FellowshipType}, membership::{Participant, StatusInPlatform}}};

use super::jwt::middleware::JwtGuard;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
pub fn about() -> &'static str {
    "This is a test!"
}


#[log_execution(roles="admin")]
#[get("/fellow/<uuid>")]
pub fn fellow_endpoint(jwt_guard: JwtGuard, uuid: &str) -> Result<Json<Fellow>, status::NotFound<String>> {
    let pool: std::sync::Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>> = create_pool();

    match find_fellow(&pool, &uuid) {
        Ok(m) => Ok(Json(m)),
        Err(_) => Err(status::NotFound("Membership not found".to_string())),
    }
}

#[post("/fellow", format = "application/json", data = "<fellow>")]
pub fn create_fellow_endpoint(fellow: Json<NewFellowForm>) -> Result<status::Created<String>, status::Custom<String>>  {
    let pool = create_pool();

    let status = match StatusInPlatform::from_str(&fellow.status_in_platform) {
        Ok(e) => e,
        Err(_) => return Err(status::Custom(Status::BadRequest, "Invalid status in platform".to_string()))
    };

    let fellowship_type = match FellowshipType::from_str(&fellow.fellowship_type) {
        Ok(e) => e,
        Err(_) => return Err(status::Custom(Status::BadRequest, "Invalid fellowship type".to_string()))
    };

    let uuid_gen: Uuid = Uuid::new_v4();

    match create_fellow(&pool,&uuid_gen.to_string(), &status, &fellowship_type) {
        Ok(fellow) => {
            let location = format!("/api/fellow/{}", fellow.get_membership().get_code());
            Ok(status::Created::new(location).body(format!("Fellow created with ID: {}", fellow.get_membership().get_code())))
        },
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}


#[derive(FromForm, Serialize, Deserialize)]
pub struct NewFellowForm {
    status_in_platform: String,
    fellowship_type: String
}