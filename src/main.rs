#[macro_use]
extern crate diesel;

mod domain;
mod dao;
mod db;

use db::initial_data::create_initial_data;

use tokio::main;

#[main]
async fn main() {

    create_initial_data().await;

}