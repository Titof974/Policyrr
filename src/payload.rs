use crate::{action::Action, resource::Resource, role::Role};

#[derive(Debug,Clone)]
pub struct Payload {
    pub actions: Vec<Action>,
    pub resource: Resource,
    pub roles: Vec<Role>,
}

impl Payload {
    pub fn new(resource: Resource, roles: Vec<Role>, actions: Vec<Action>) -> Self {
        Payload { actions, resource, roles }
    }
}