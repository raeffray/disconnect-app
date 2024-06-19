use std::io::Write;
use diesel::prelude::*;
use typed_builder::TypedBuilder;
use crate::{db::schema::memberships, domain::membership::StatusInPlatform};

use diesel::{deserialize::{self, FromSql}, serialize::{self, IsNull, Output, ToSql}, sql_types::Text};

#[derive(Debug, TypedBuilder, Queryable, Identifiable, Selectable)]
#[table_name = "memberships"]
pub struct MembershipModel {
    pub id: i32,
    pub code: String,
    pub status_in_platform: StatusInPlatform
}

// ---------

impl ToSql<Text, diesel::pg::Pg> for StatusInPlatform {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        match *self {
            StatusInPlatform::Active => out.write_all(b"Active")?,
            StatusInPlatform::Inactive => out.write_all(b"Inactive")?,
            StatusInPlatform::Blocked => out.write_all(b"Blocked")?,
        }
        Ok(IsNull::No)

    }
}

impl FromSql<Text, diesel::pg::Pg> for StatusInPlatform {
    fn from_sql(bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Active" => Ok(StatusInPlatform::Active),
            b"Inactive" => Ok(StatusInPlatform::Inactive),
            b"Blocked" => Ok(StatusInPlatform::Blocked),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
