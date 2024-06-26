use rocket::serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;
use diesel::prelude::*;

use crate::db::schema::members;

#[derive(Debug, TypedBuilder, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(MembershipModel))]
#[table_name = "members"]
pub struct MemberModel {
    id: i32,
    name: String,
    membership_id: i32
}

impl MemberModel {
    pub fn get_id(&self) -> &i32{
        &self.id
    }
    pub fn get_name(&self) -> &String{
        &self.name
    }
    pub fn get_membership_id(&self) -> &i32{
        &self.membership_id
    }
}
