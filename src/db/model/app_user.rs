use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::users;


#[derive(Debug, Queryable, Serialize, Deserialize, Selectable)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub secret: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserModel<'a> {
    pub user_id: &'a str,
    pub secret: &'a str,
}