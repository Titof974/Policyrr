use crate::{action::Action, resource::Resource, role::Role, payload::Payload, effect::Effect};
use regex::Regex;
use serde::{Deserialize, Serialize};
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Rule {
    pub actions: Vec<Action>,
    pub resource: Resource,
    pub effect: Effect,
    pub roles: Vec<Role>,
}

impl Rule {
    pub fn new(resource: Resource, effect: Effect, actions: Vec<Action>, roles: Vec<Role>) -> Rule {
        Rule {
            actions: actions.clone(),
            effect: effect.clone(),
            resource: resource.clone(),
            roles: roles.clone(),
        }
    }

    pub fn verify(&self, payload: Payload) -> bool {
        let resource_formated_string = "^".to_string() + &self.resource.value + "$";
        let re_resource = Regex::new(&resource_formated_string).expect("Invalid resource regex");
        let check_resource = re_resource.is_match(&payload.resource.value);
        let check_roles = self.roles.iter().all(|role| {
            payload.roles.iter().any(|payload_role| payload_role.value.eq(&role.value))
        });
        let check_actions = self.actions.iter().all(|action| {
            let action_formated_string = "^".to_string() + &action.value + "$";
            let re_action = Regex::new(&action_formated_string).expect("Invalid resource regex");
            payload.actions.iter().any(|payload_action| {
                re_action.is_match(&payload_action.value)
            })
        });
        check_resource && check_actions && check_roles
    }
}

