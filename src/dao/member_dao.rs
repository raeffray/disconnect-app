use std::error::Error;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Connection, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};

use crate::{
    db::{
        model::{
            member_model::MemberModel,
            membership_model::MembershipModel,
        },
        pool::DbPool,
        schema::members,
    },
    domain::{
        member::Member,
        membership::{Membership, StatusInPlatform},
    },
};

use super::membership_dao::create_membership;

pub fn create_member(
    pool: &DbPool,
    new_code: &str,
    status: &StatusInPlatform,
    new_name: &str,
) -> Result<Member, Box<dyn Error + Send + Sync>> {
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .expect("Failed to get a connection from the pool");

    connection.transaction(|c| {
        let new_membership_model: MembershipModel = create_membership(pool, new_code, status)?;

        let new_member_model: MemberModel = diesel::insert_into(members::table)
            .values((
                members::name.eq(new_name),
                members::membership_id.eq(new_membership_model.id),
            ))
            .returning(MemberModel::as_returning())
            .get_result(c)?;

        Ok(Member::builder()
            .membership(
                Membership::builder()
                    .id(new_membership_model.id)
                    .status_in_platform(new_membership_model.status_in_platform)
                    .code(new_membership_model.code)
                    .build(),
            )
            .id(new_member_model.get_id().clone())
            .name(new_member_model.get_name().clone())
            .build())
    })
}
