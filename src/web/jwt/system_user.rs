use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::{system_user_roles, systemusers};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct SystemUser {
    pub id: i32,
    pub user_id: String,
    pub secret: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = systemusers)]
pub struct NewSystemUserModel<'a> {
    pub user_id: &'a str,
    pub secret: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(SystemUser)]
#[belongs_to(Role)]
#[diesel(table_name = system_user_roles)]
#[diesel(primary_key(system_user_id, role_id))] 
pub struct SystemUserRole {
    pub system_user_id: i32,
    pub role_id: i32,
}