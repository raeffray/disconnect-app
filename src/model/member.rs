use typed_builder::TypedBuilder;

use super::membership::{Membership, Participant};

#[derive(Debug, TypedBuilder)]
pub struct Member {
    membership: Membership,
}

impl Participant for Member {

    fn get_membership(&self) -> &Membership {
        &self.membership
    }

 
}