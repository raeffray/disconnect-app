use typed_builder::TypedBuilder;

use super::membership::{Membership, Participant};

#[derive(Debug, TypedBuilder)]
pub struct Member {
    id: i32,
    membership: Membership,
    name: String
}

impl Member {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl Participant for Member {

    fn get_id(&self) -> &i32 {
        &self.id
    }
    
    fn get_membership(&self) -> &Membership {
        &self.membership
    }

}