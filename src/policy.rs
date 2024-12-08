use crate::{effect::Effect, payload::Payload, rule::Rule};

pub(crate) struct Policy {
    rules: Vec<Rule>,
}

impl Policy {
    pub fn new(rules: Vec<Rule>) -> Policy {
        Policy { rules: rules.clone() }
    }

    pub fn verify(&self, payload: Payload) -> bool {
        !self.rules.iter().filter(|rule| rule.effect == Effect::DENY).any(|rule| rule.clone().verify(payload.clone())) && 
        self.rules.iter().filter(|rule| rule.effect == Effect::ALLOW).any(|rule| rule.verify(payload.clone()))
    }
}