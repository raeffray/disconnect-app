use rocket::{get, response::{status}, serde::json::Json};
use crate::{dao::membership_dao::find_membership, db::model::membership_model::MembershipModel};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
pub fn about() -> &'static str {
    "This is a test!"
}

#[get("/fellow/<uuid>")]
pub fn fellow_endpoint(uuid: String) -> Result<Json<MembershipModel>, status::NotFound<String>> {
    match find_membership(&uuid) {
        Ok(m) => Ok(Json(m)),
        Err(_) => Err(status::NotFound("Membership not found".to_string())),
    }
}
