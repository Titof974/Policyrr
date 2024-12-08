use crate::{action::Action, resource::Resource, role::Role};

use serde::{Deserialize, Serialize};
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Rule {
    pub actions: Vec<Action>,
    pub resource: Resource,
    pub roles: Vec<Role>,
}

impl Rule {
    pub fn new(resource: Resource, actions: Vec<Action>, roles: Vec<Role>) -> Rule {
        Rule {
            actions: actions.clone(),
            resource: resource.clone(),
            roles: roles.clone(),
        }
    }
}

