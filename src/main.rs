mod model;
mod dao;
mod db;

use std::result;

use model::person::{PersonCommon, StatusInPlatform};
use model::user::User;
use model::volunteer::{Volunteer, VolunteerType};
use tokio::main;

#[main]
async fn main() {

    let pool = db::pool::create_pool();

    let result = dao::userdao::get_user(pool, "test").await;

    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e)
    }


    // let volunteer: Volunteer = Volunteer::builder()
    //     .person_common(
    //         PersonCommon::builder()
    //             .code(String::from("V123"))
    //             .status_in_platform(StatusInPlatform::Active)
    //             .build(),
    //     )
    //     .volunter_type(VolunteerType::HealthWorker)
    //     .build();

    // println!("{:?}", volunteer);

    // let user: User = User::builder()
    //     .person_common(
    //         PersonCommon::builder()
    //             .code(String::from("345"))
    //             .status_in_platform(StatusInPlatform::Active)
    //             .build(),
    //     )
    //     .build();

    // println!("{:?}", user);

    // let test: [u8; 4] = [1,2,3, 20];


    // let filtered: Vec<&u8> = test.iter().filter(|&&x| x == 20).collect();

    // match filtered.first() {
    //     Some(x) => println!("{}", x),
    //     _ => println!("not found")
    // }


}
