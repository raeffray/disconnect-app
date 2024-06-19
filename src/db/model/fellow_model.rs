use std::io::Write;
use diesel::prelude::*;
use typed_builder::TypedBuilder;
use crate::{db::schema::fellows, domain::fellow::FellowshipType};

use diesel::{deserialize::{self, FromSql}, serialize::{self, IsNull, Output, ToSql}, sql_types::Text};

#[derive(Debug, TypedBuilder, Queryable, Identifiable, Selectable)]
#[table_name = "fellows"]
pub struct FellowModel {
    pub id: i32,
    pub fellowship_type: FellowshipType
}

impl FellowModel {
    pub fn get_id(&self) -> &i32{
        &self.id
    }
    pub fn get_fellowship_type(&self) -> &FellowshipType{
        &self.fellowship_type
    }
}


impl ToSql<Text, diesel::pg::Pg> for FellowshipType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        match *self {
            FellowshipType::HealthWorker => out.write_all(b"HealthWorker")?,
            FellowshipType::Promoter => out.write_all(b"Promoter")?,
        }
        Ok(IsNull::No)

    }
}

impl FromSql<Text, diesel::pg::Pg> for FellowshipType {
    fn from_sql(bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"HealthWorker" => Ok(FellowshipType::HealthWorker),
            b"Promoter" => Ok(FellowshipType::Promoter),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
