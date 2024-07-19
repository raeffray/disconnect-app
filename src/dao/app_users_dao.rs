use diesel::{
     r2d2::{ConnectionManager, PooledConnection}, ExpressionMethods, PgConnection, SelectableHelper
};
use std::error::Error;

use diesel::{
     QueryDsl, RunQueryDsl
};

use crate::db::{model::app_user::User, pool::DbPool, schema::users};

pub fn find_app_user(pool: &DbPool, user_id: &str, secret: &str) -> Result<User, diesel::result::Error> {
    
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let found_user: User = users::table
        .filter(users::user_id.eq(user_id))
        .filter(users::secret.eq(secret))
        .first::<User>(&mut connection)?;

    Ok(found_user)
}