use serde::{Deserialize, Serialize};

use crate::types::Community;

/// Describes a community chat added to a community.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatAdded {
    /// The community to which the chat was added.
    pub community: Community,
}

#[cfg(test)]
mod tests {
    use crate::types::CommunityChatAdded;

    #[test]
    fn deserializes_added_community() {
        let added: CommunityChatAdded =
            serde_json::from_str(r#"{"community":{"id":1,"name":"Rust"}}"#).unwrap();
        assert_eq!(added.community.id, 1);
    }
}
