use typed_builder::TypedBuilder;

pub trait Person {
    fn get_person_common(&self) -> &PersonCommon;
}

 #[derive(Debug)]
pub enum StatusInPlatform {
    Active,
    Inactive,
    Blocked
}

#[derive(Debug, TypedBuilder)]
pub struct PersonCommon {
    code: String,
    status_in_platform: StatusInPlatform
}

impl PersonCommon {

    fn get_code(&self) -> &str {
        &self.code
    }

    fn get_status_in_platform(&self) -> &StatusInPlatform {
        &self.status_in_platform
    }

}