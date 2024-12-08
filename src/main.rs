use resource::Resource;
use action::Action;
use role::Role;
use rule::Rule;

mod resource;
mod action;
mod role;
mod rule;

fn main() {
    let res = Resource::new("hello".to_owned());
    let action = Action::new("test".to_owned());
    let role = Role::new("test".to_owned());

    let rule = Rule::new(res, vec![action], vec![role]);
    println!("{:?}", rule);
    let test = serde_json::to_string(&rule).unwrap();
    println!("{:?}", test);
    let v: Rule = serde_json::from_str(test.as_str()).unwrap();
    println!("{:?}", v);
}
