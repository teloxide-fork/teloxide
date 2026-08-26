use serde::{Deserialize, Serialize};

/// Represents an HTTP link.
///
/// [The official docs](https://core.telegram.org/bots/api#link).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Link {
    /// URL of the link.
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{"url":"https://example.com"}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert_eq!(link.url, "https://example.com");
    }
}
