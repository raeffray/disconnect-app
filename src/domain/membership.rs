use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use diesel::sql_types::Text;

pub trait Participant {
    fn get_membership(&self) -> &Membership;
    fn get_id(&self) -> &i32;
}


#[derive(Debug,AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub enum StatusInPlatform {
    Active,
    Inactive,
    Blocked
}

#[derive(Debug, TypedBuilder)]
pub struct Membership {
    id: i32,
    code: String,
    status_in_platform: StatusInPlatform
}

impl Membership {
    
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_status_in_platform(&self) -> &StatusInPlatform {
        &self.status_in_platform
    }
    
}