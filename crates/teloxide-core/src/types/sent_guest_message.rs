use serde::{Deserialize, Serialize};

/// This object contains information about a guest message sent by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#sentguestmessage).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: String,
}

#[cfg(test)]
mod tests {
    use crate::types::SentGuestMessage;

    #[test]
    fn deserialize() {
        let sent: SentGuestMessage =
            serde_json::from_str(r#"{"inline_message_id":"guest-message"}"#).unwrap();

        assert_eq!(sent.inline_message_id, "guest-message");
    }
}
