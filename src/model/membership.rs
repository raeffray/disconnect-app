use typed_builder::TypedBuilder;

pub trait Participant {
    fn get_membership(&self) -> &Membership;
}

 #[derive(Debug)]
pub enum StatusInPlatform {
    Active,
    Inactive,
    Blocked
}

#[derive(Debug, TypedBuilder)]
pub struct Membership {
    code: String,
    status_in_platform: StatusInPlatform
}