use std::io::Write;
use diesel::prelude::*;
use typed_builder::TypedBuilder;
use rocket::serde::{Serialize, Deserialize};
use crate::{db::schema::memberships, domain::membership::StatusInPlatform};

use diesel::{deserialize::{self, FromSql}, serialize::{self, IsNull, Output, ToSql}, sql_types::Text};

#[derive(Debug, TypedBuilder, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
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
            StatusInPlatform::ACTIVE => out.write_all(b"Active")?,
            StatusInPlatform::INACTIVE => out.write_all(b"Inactive")?,
            StatusInPlatform::BLOCKED => out.write_all(b"Blocked")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::pg::Pg> for StatusInPlatform {
    fn from_sql(bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Active" => Ok(StatusInPlatform::ACTIVE),
            b"Inactive" => Ok(StatusInPlatform::INACTIVE),
            b"Blocked" => Ok(StatusInPlatform::BLOCKED),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
