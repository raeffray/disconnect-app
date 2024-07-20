use diesel::{
     r2d2::{self, ConnectionManager, PooledConnection}, ExpressionMethods, JoinOnDsl, PgConnection
};

use crate::{db::schema::{roles, system_user_roles, systemusers}, web::jwt::system_user::SystemUser};

use diesel::{
     QueryDsl, RunQueryDsl
};

use crate::db::pool::DbPool;

pub fn find_app_user(pool: &DbPool, user_id: &str, secret: &str) -> Result<SystemUser, diesel::result::Error> {
    
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let found_user: SystemUser = systemusers::table
        .filter(systemusers::user_id.eq(user_id))
        .filter(systemusers::secret.eq(secret))
        .first::<SystemUser>(&mut connection)?;

    Ok(found_user)
}

pub fn find_system_user(pool: &DbPool, user_id: &str, secret: &str) -> Result<(SystemUser, Vec<String>), diesel::result::Error> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

        let user: SystemUser = systemusers::table
        .filter(systemusers::user_id.eq(user_id))
        .filter(systemusers::secret.eq(secret))
        .first::<SystemUser>(&mut connection)?;

    let user_roles: Vec<String> = system_user_roles::table
        .inner_join(roles::table.on(system_user_roles::role_id.eq(roles::id)))
        .filter(system_user_roles::system_user_id.eq(user.id))
        .select(roles::name)
        .load::<String>(&mut connection)?;

    Ok((user, user_roles))
}
