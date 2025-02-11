#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod web;
mod dao;
mod db;
mod domain;
mod utils;

use web::routes::{about, create_fellow_endpoint, fellow_endpoint, index};
use web::jwt::route::generate_jwt_token_route;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index, about, fellow_endpoint, create_fellow_endpoint, generate_jwt_token_route])
}
