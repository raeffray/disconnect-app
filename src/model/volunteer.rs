use super::person::{Person, PersonCommon, StatusInPlatform};
use typed_builder::TypedBuilder;


#[derive(Debug)]
pub enum VolunteerType {
    HealthWorker,
    Promoter,
}

#[derive(Debug, TypedBuilder)]
pub struct Volunteer {
    person_common: PersonCommon,    
    volunter_type: VolunteerType,
}

impl Volunteer {

    pub fn get_type(&self) -> &VolunteerType {
        &self.volunter_type
    }

}

impl Person for Volunteer {

   fn get_person_common(&self) -> &PersonCommon {
       &self.person_common
   }

}
