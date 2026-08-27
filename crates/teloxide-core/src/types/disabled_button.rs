use serde::{Deserialize, Serialize};

/// Represents a disabled button.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct DisabledButton {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_as_empty_object() {
        assert_eq!(serde_json::to_value(DisabledButton {}).unwrap(), serde_json::json!({}));
    }
}
