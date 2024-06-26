#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod web;
mod dao;
mod db;
mod domain;

use web::routes::{about, create_fellow_endpoint, fellow_endpoint, index};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, about, fellow_endpoint, create_fellow_endpoint])
}
