use crate::types::UserId;
use serde::{Deserialize, Serialize};

/// Parameters describing an ephemeral message recipient and callback behavior.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct EphemeralMessageParameters {
    /// Identifier of the user that can receive the ephemeral message.
    pub receiver_user_id: UserId,

    /// Identifier of the callback query that resulted in the message.
    pub callback_query_id: Option<String>,

    /// `true` to replace the message that caused the callback query.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replace_callback_query_message: bool,
}

impl EphemeralMessageParameters {
    pub const fn new(receiver_user_id: UserId) -> Self {
        Self { receiver_user_id, callback_query_id: None, replace_callback_query_message: false }
    }

    pub fn callback_query_id(mut self, value: impl Into<String>) -> Self {
        self.callback_query_id = Some(value.into());
        self
    }

    pub const fn replace_callback_query_message(mut self, value: bool) -> Self {
        self.replace_callback_query_message = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_all_fields() {
        let params: EphemeralMessageParameters = serde_json::from_str(
            r#"{"receiver_user_id":1,"callback_query_id":"q","replace_callback_query_message":true}"#,
        )
        .unwrap();

        assert_eq!(params.receiver_user_id, UserId(1));
        assert_eq!(serde_json::to_value(params).unwrap()["callback_query_id"], "q");
    }
}
