use serde::{Deserialize, Serialize};

/// A Telegram community.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Community {
    /// Unique identifier of the community.
    pub id: i64,
    /// Name of the community.
    pub name: String,
}

impl Community {
    pub fn new(id: i64, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }

    pub const fn id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Community;

    #[test]
    fn deserializes_community() {
        let community: Community = serde_json::from_str(r#"{"id":1,"name":"Rust"}"#).unwrap();
        assert_eq!(community.name, "Rust");
    }
}
