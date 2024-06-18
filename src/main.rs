#[macro_use]
extern crate diesel;

mod domain;
mod dao;
mod db;

use db::initialdata::create_initial_data;

use tokio::main;

#[main]
async fn main() {

    create_initial_data().await;

}