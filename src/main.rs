#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod web;
mod dao;
mod db;
mod domain;

use web::routes::{about, fellow_endpoint, index};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, about, fellow_endpoint])
}
