use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use diesel::sql_types::Text;

use super::membership::{Membership, Participant};

#[derive(Debug, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub enum FellowshipType {
    HealthWorker,
    Promoter,
}

#[derive(Debug, TypedBuilder)]
pub struct Fellow {
    id: i32,
    membership: Membership,
    fellowship_type: FellowshipType,
}

impl Fellow {
    pub fn get_type(&self) -> &FellowshipType {
        &self.fellowship_type
    }
}

impl Participant for Fellow {

    fn get_id(&self) -> &i32 {
        return &self.id
    }

    fn get_membership(&self) -> &Membership {
        &self.membership
    }

}