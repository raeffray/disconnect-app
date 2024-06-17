mod model;
mod dao;
mod db;

use std::result;

use model::{fellow::{Fellow, FellowshipType}, member::Member, membership::{Membership, StatusInPlatform}};
use tokio::main;

#[main]
async fn main() {

    // let pool = db::pool::create_pool();

    // let result = dao::userdao::get_user(pool, "test").await;

    // match result {
    //     Ok(n) => println!("{}", n),
    //     Err(e) => println!("{}", e)
    // }

    let fellow: Fellow = Fellow::builder()
        .membership(
            Membership::builder()
                .code(String::from("V123"))
                .status_in_platform(StatusInPlatform::Active)
                .build(),
        )
        .fellowship_type(FellowshipType::HealthWorker)
        .build();

    println!("{:?}", fellow);

    let member: Member = Member::builder()
        .membership(
            Membership::builder()
                .code(String::from("345"))
                .status_in_platform(StatusInPlatform::Active)
                .build(),
        )
        .build();

    println!("{:?}", member);

}
