use serde::{Deserialize, Serialize};
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Effect {
    ALLOW,
    DENY
}