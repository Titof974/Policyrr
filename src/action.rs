use serde::{Serialize, Serializer, de::Visitor, Deserialize, Deserializer};
#[derive(Debug,Clone)]
pub struct Action {
    pub value: String,
}

impl Action {
    pub fn new(value: String) -> Action {
        Action { 
            value: value.to_owned()
         }
    }
}


impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        deserializer.deserialize_string(ActionVisitor)
    }
}

struct ActionVisitor;

impl<'de> Visitor<'de> for ActionVisitor {
    type Value = Action;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Action::new(v.to_string()))
    }


    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Action::new(v))
    }
}