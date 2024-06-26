use rocket::{get, response::{status}, serde::json::Json};
use crate::{dao::{fellow_dao::find_fellow, membership_dao::find_membership}, db::{model::{fellow_model::FellowModel, membership_model::MembershipModel}, pool::{self, create_pool}}};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
pub fn about() -> &'static str {
    "This is a test!"
}

#[get("/fellow/<uuid>")]
pub fn fellow_endpoint(uuid: String) -> Result<Json<FellowModel>, status::NotFound<String>> {
    let pool = create_pool();

    match find_fellow(&pool, &uuid) {
        Ok(m) => Ok(Json(m)),
        Err(_) => Err(status::NotFound("Membership not found".to_string())),
    }
}


#[post("/fellow/")]
pub fn create_fellow_endpoint(uuid: String) -> Result<Json<FellowModel>, status::NotFound<String>> {
    let pool = create_pool();

    match find_fellow(&pool, &uuid) {
        Ok(m) => Ok(Json(m)),
        Err(_) => Err(status::NotFound("Membership not found".to_string())),
    }
}
