use resource::Resource;
use action::Action;
use role::Role;
use rule::Rule;
use payload::Payload;
use effect::Effect;
use policy::Policy;
use std::time::{Duration, Instant};

mod resource;
mod action;
mod role;
mod rule;
mod payload;
mod effect;
mod policy;

fn main() {
    
    let res = Resource::new("hel:lo:.*".to_owned());
    let action = Action::new("test".to_owned());
    let role = Role::new("test".to_owned());

    let rule = Rule::new(res.clone(), Effect::ALLOW, vec![action.clone()], vec![role.clone()]);
    let rule2 = Rule::new(res.clone(), Effect::DENY, vec![action.clone()], vec![role.clone()]);
    // println!("{:?}", rule);
    // let test = serde_json::to_string(&rule).unwrap();
    // println!("{:?}", test);
    // let v: Rule = serde_json::from_str(test.as_str()).unwrap();
    // println!("{:?}", v);

    let res = Resource::new("hel:lo:test".to_owned());
    let action = Action::new("test".to_owned());
    let role = Role::new("test".to_owned());
    let payload = Payload::new(res.clone(), vec![role.clone()], vec![action.clone()]);
    let now = Instant::now();
    println!("{:?}", rule.verify(payload.clone()));
    println!("{}", now.elapsed().as_millis());

    let pol = Policy::new(vec![rule.clone(), rule2.clone()]);
    println!("{:?}", pol.verify(payload.clone()));

}
