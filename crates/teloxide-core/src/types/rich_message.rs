use serde::{Deserialize, Serialize};

use crate::types::RichBlock;

/// Rich formatted message.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessage).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    /// Content of the message.
    pub blocks: Vec<RichBlock>,

    /// `true`, if the rich message must be shown right-to-left.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_rtl: bool,
}

/// Describes a rich message to be sent.
///
/// Exactly **one** of the fields `html` or `markdown` must be used (not
/// enforced by the type).
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    /// Content of the rich message to send described using HTML formatting.
    pub html: Option<String>,

    /// Content of the rich message to send described using Markdown
    /// formatting.
    pub markdown: Option<String>,

    /// Pass `true` if the rich message must be shown right-to-left.
    pub is_rtl: Option<bool>,

    /// Pass `true` to skip automatic detection of entities (e.g., URLs,
    /// email addresses, username mentions, hashtags, cashtags, bot commands,
    /// or phone numbers) in the text.
    pub skip_entity_detection: Option<bool>,
}

/// Represents the content of a rich message to be sent as the result of an
/// inline query.
///
/// Named `InputMessageContentRichMessage` (not the literal Telegram name
/// `InputRichMessageContent`) to match this crate's `InputMessageContent*`
/// naming convention for the other `InputMessageContent` variants.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagecontent).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMessageContentRichMessage {
    /// The message to be sent.
    pub rich_message: InputRichMessage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RichBlockParagraph;

    #[test]
    fn deserialize_rich_message_end_to_end() {
        let json = r#"{
            "blocks":[{"type":"paragraph","text":"hello"}],
            "is_rtl":true
        }"#;
        let message: RichMessage = serde_json::from_str(json).unwrap();

        assert_eq!(
            message,
            RichMessage {
                blocks: vec![RichBlock::Paragraph(RichBlockParagraph {
                    text: crate::types::RichText::Plain("hello".to_owned())
                })],
                is_rtl: true,
            }
        );
    }
}
