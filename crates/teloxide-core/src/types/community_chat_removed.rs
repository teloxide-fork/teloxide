use serde::{Deserialize, Serialize};

/// Describes a community chat removed from a community.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatRemoved {}

#[cfg(test)]
mod tests {
    use crate::types::CommunityChatRemoved;

    #[test]
    fn deserializes_empty_object() {
        serde_json::from_str::<CommunityChatRemoved>("{}").unwrap();
    }
}
