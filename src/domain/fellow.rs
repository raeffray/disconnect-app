use std::str::FromStr;

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

impl FromStr for FellowshipType {
    type Err = ();
    fn from_str(input: &str) -> Result<FellowshipType, Self::Err> {
        match input {
            "HEALTH_WORKER" => Ok(FellowshipType::HealthWorker),
            "PROMOTER" => Ok(FellowshipType::Promoter),
            _ => Err(()),
        }
    }
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
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