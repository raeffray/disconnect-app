use typed_builder::TypedBuilder;

use super::membership::{Membership, Participant};


#[derive(Debug)]
pub enum FellowshipType {
    HealthWorker,
    Promoter,
}

#[derive(Debug, TypedBuilder)]
pub struct Fellow {
    membership: Membership,
    fellowship_type: FellowshipType,
}

impl Fellow {

    pub fn get_type(&self) -> &FellowshipType {
        &self.fellowship_type
    }

}

impl Participant for Fellow {

    fn get_membership(&self) -> &Membership {
        &self.membership
    }

}
