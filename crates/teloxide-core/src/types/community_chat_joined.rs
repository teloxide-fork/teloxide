use crate::types::Community;
use serde::{Deserialize, Serialize};

/// Describes a community chat joined by the user.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatJoined {
    /// The community that the chat joined.
    pub community: Community,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_community() {
        let joined: CommunityChatJoined =
            serde_json::from_str(r#"{"community":{"id":1,"name":"Rust"}}"#).unwrap();
        assert_eq!(joined.community.name, "Rust");
    }
}
