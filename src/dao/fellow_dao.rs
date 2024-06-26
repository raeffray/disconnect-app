use std::error::Error;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Connection, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};

use crate::{
    db::{
        model::{fellow_model::FellowModel, membership_model::MembershipModel},
        pool::DbPool,
        schema::fellows,
    },
    domain::{
        fellow::{Fellow, FellowshipType},
        membership::{Membership, StatusInPlatform},
    },
};

use diesel::prelude::*; // Brings the query DSL methods into scope

use super::membership_dao::{create_membership, find_membership};

pub fn create_fellow(
    pool: &DbPool,
    new_code: &str,
    status: &StatusInPlatform,
    new_fellowship_type: &FellowshipType,
) -> Result<Fellow, Box<dyn Error + Send + Sync>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    connection.transaction(|c| {
        let new_membership_model: MembershipModel = create_membership(pool, new_code, status)?;

        let new_fellow_model: FellowModel = diesel::insert_into(fellows::table)
            .values((
                fellows::fellowship_type.eq(new_fellowship_type),
                fellows::membership_id.eq(new_membership_model.id),
            ))
            .returning(FellowModel::as_returning())
            .get_result(c)?;

        Ok(Fellow::builder()
            .membership(
                Membership::builder()
                    .id(new_membership_model.id)
                    .status_in_platform(new_membership_model.status_in_platform)
                    .code(new_membership_model.code)
                    .build(),
            )
            .id(*new_fellow_model.get_id())
            .fellowship_type(new_fellow_model.get_fellowship_type().clone())
            .build())
    })
}

pub fn find_fellow(
    pool: &DbPool,
    membership_code: &str,
) -> Result<Fellow, Box<dyn Error + Send + Sync>> {
    let membership: MembershipModel = find_membership(pool, membership_code).unwrap();

    println!("{:?}", membership.id);

    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let found_fellow: FellowModel = fellows::table
        .filter(fellows::membership_id.eq(membership.id))
        .first::<FellowModel>(&mut connection)?;

    let fellow = Fellow::builder()
        .id(found_fellow.id)
        .fellowship_type(found_fellow.fellowship_type)
        .membership(
            Membership::builder()
                .id(membership.id)
                .status_in_platform(membership.status_in_platform)
                .code(membership.code)
                .build(),
        )
        .build();

    Ok(fellow)
}
