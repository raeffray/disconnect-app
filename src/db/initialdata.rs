use std::sync::Arc;


use crate::{dao::memberdao::create_member, domain::membership::StatusInPlatform};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;

use super::pool::create_pool;

pub async fn create_initial_data() {
    let uuid = Uuid::new_v4();

    let pool: Arc<Pool<ConnectionManager<PgConnection>>> = create_pool();

    match create_member(&pool, &uuid.to_string(), &StatusInPlatform::Active, "test") {
        Ok(u) => println!("Member Created! [{:?}]", u),
        Err(e) => println!("Error [{}]", e),
    }

}
