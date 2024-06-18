use std::{error::Error, sync::Arc};

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::{db::{model::membership::MembershipModel, pool::{create_pool, DbPool}, schema::*}, domain::membership::StatusInPlatform};

pub fn create_membership(pool: &DbPool, new_code: &str, status: &StatusInPlatform) -> Result<MembershipModel, Box<dyn Error + Send + Sync>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let new_membershipodel = diesel::insert_into(memberships::table)
        .values((
            memberships::code.eq(new_code),
            memberships::status_in_platform.eq(status)
        ))
        .returning(MembershipModel::as_returning())
        .get_result(&mut connection)?;

    Ok(new_membershipodel)
}


pub fn find_membership(
    membership_code: &str,
) -> Result<MembershipModel, Box<dyn Error + Send + Sync>> {
    let pool: Arc<Pool<ConnectionManager<PgConnection>>> = create_pool();
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let found_membership = memberships::table
        .filter(memberships::code.eq(membership_code))
        .select(MembershipModel::as_select())
        .get_result(&mut connection)?;

    Ok(found_membership)
}
