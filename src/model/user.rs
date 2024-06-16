use typed_builder::TypedBuilder;

use super::person::{Person, PersonCommon, StatusInPlatform};

#[derive(Debug,TypedBuilder)]
pub struct User {

    person_common: PersonCommon
    
}

impl User {
    
}

impl Person for User {
    fn get_person_common(&self) -> &PersonCommon {
        &self.person_common
    }
}


