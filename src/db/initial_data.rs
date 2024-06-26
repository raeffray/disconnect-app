// use std::sync::Arc;


// use crate::{dao::{fellow_dao::create_fellow, member_dao::create_member}, domain::{fellow::{Fellow, FellowshipType}, membership::StatusInPlatform}};
// use diesel::{
//     r2d2::{ConnectionManager, Pool},
//     PgConnection,
// };
// use uuid::Uuid;

// use super::pool::create_pool;

// pub async fn create_initial_data() {
//     let uuid: Uuid = Uuid::new_v4();

//     let pool: Arc<Pool<ConnectionManager<PgConnection>>> = create_pool();

//     match create_member(&pool, &uuid.to_string(), &StatusInPlatform::ACTIVE, "test") {
//         Ok(u) => println!("Member Created! [{:?}]", u),
//         Err(e) => println!("Error [{}]", e),
//     }

//     match create_fellow(&pool, &uuid.to_string(), &StatusInPlatform::ACTIVE, &FellowshipType::HealthWorker) {
//         Ok(u) => println!("Member Created! [{:?}]", u),
//         Err(e) => println!("Error [{}]", e),
//     }

// }
