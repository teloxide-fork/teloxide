use serde::{de::Error as _, Deserialize, Deserializer, Serialize};

use crate::types::{InputFile, InputFileLike, InputRichBlock, InputRichMessageMedia, RichBlock};

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
/// Exactly **one** of the fields `html`, `markdown`, or `blocks` must be used
/// (not enforced by the type).
///
/// Note: equality comparisons for values carrying freshly-constructed
/// uploadable file content are not stable; see the [`PartialEq`] impl docs
/// for details.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    /// Content of the rich message to send described using HTML formatting.
    pub html: Option<String>,

    /// Content of the rich message to send described using Markdown
    /// formatting.
    pub markdown: Option<String>,

    /// Content of the rich message as structured blocks.
    pub blocks: Option<Vec<InputRichBlock>>,

    /// Media referenced by structured blocks in the rich message.
    pub media: Option<Vec<InputRichMessageMedia>>,

    /// Pass `true` if the rich message must be shown right-to-left.
    pub is_rtl: Option<bool>,

    /// Pass `true` to skip automatic detection of entities (e.g., URLs,
    /// email addresses, username mentions, hashtags, cashtags, bot commands,
    /// or phone numbers) in the text.
    pub skip_entity_detection: Option<bool>,
}

impl InputFileLike for InputRichMessage {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flatten().for_each(|media| media.media.copy_into(into));
        self.blocks.iter().flatten().for_each(|block| block.copy_into(into));
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flatten().for_each(|media| media.media.move_into(into));
        self.blocks.iter_mut().flatten().for_each(|block| block.move_into(into));
    }
}

impl InputFileLike for Option<InputRichMessage> {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        if let Some(message) = self {
            message.copy_into(into);
        }
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        if let Some(message) = self {
            message.move_into(into);
        }
    }
}

/// Equality is only meaningful/stable for values that don't carry
/// freshly-constructed uploadable file content (i.e. `html`/`markdown`-only
/// messages, or `blocks`/`media` that only reference existing `file_id`s or
/// URLs). `InputFile` variants that need to be uploaded (`memory`/`file`/
/// `read`) lazily generate a random attachment id on first serialization,
/// cached per-instance; two independently constructed values carrying
/// otherwise-identical new-upload `InputFile` content may therefore compare
/// unequal, because each one generates its own random attachment id.
impl PartialEq for InputRichMessage {
    fn eq(&self, other: &Self) -> bool {
        serde_json::to_value(self).ok() == serde_json::to_value(other).ok()
    }
}

impl<'de> Deserialize<'de> for InputRichMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Fields {
            html: Option<String>,
            markdown: Option<String>,
            #[serde(default)]
            blocks: Option<serde::de::IgnoredAny>,
            #[serde(default)]
            media: Option<serde::de::IgnoredAny>,
            is_rtl: Option<bool>,
            skip_entity_detection: Option<bool>,
        }

        let fields = Fields::deserialize(deserializer)?;
        if fields.blocks.is_some() || fields.media.is_some() {
            return Err(D::Error::custom(
                "deserializing structured rich-message input is not supported",
            ));
        }

        Ok(Self {
            html: fields.html,
            markdown: fields.markdown,
            blocks: None,
            media: None,
            is_rtl: fields.is_rtl,
            skip_entity_detection: fields.skip_entity_detection,
        })
    }
}

/// Represents the content of a rich message to be sent as the result of an
/// inline query.
///
/// Named `InputMessageContentRichMessage` (not the literal Telegram name
/// `InputRichMessageContent`) to match this crate's `InputMessageContent*`
/// naming convention for the other `InputMessageContent` variants.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagecontent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMessageContentRichMessage {
    /// The message to be sent.
    pub rich_message: InputRichMessage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        InputFile, InputMediaAnimation, InputMediaPhoto, InputRichBlock, InputRichBlockParagraph,
        InputRichMedia, InputRichMessageMedia, RichBlockParagraph, RichText,
    };

    #[test]
    fn input_rich_message_serializes_blocks_and_media() {
        let message = InputRichMessage {
            html: None,
            markdown: None,
            blocks: Some(vec![InputRichBlock::Paragraph(InputRichBlockParagraph {
                text: RichText::Plain("hello".into()),
            })]),
            media: Some(vec![InputRichMessageMedia::new(
                "photo",
                InputRichMedia::Photo(InputMediaPhoto::new(InputFile::file_id("id".into()))),
            )]),
            is_rtl: None,
            skip_entity_detection: None,
        };

        let value = serde_json::to_value(message).unwrap();
        assert_eq!(value["blocks"][0]["type"], "paragraph");
        assert_eq!(value["media"][0]["id"], "photo");
    }

    #[test]
    fn structured_blocks_and_media_files_are_traversed() {
        let mut message = InputRichMessage {
            html: None,
            markdown: None,
            blocks: Some(vec![InputRichBlock::Animation(
                crate::types::InputRichBlockAnimation::new(InputMediaAnimation::new(
                    InputFile::memory("block"),
                )),
            )]),
            media: Some(vec![InputRichMessageMedia::new(
                "photo",
                InputRichMedia::Photo(InputMediaPhoto::new(InputFile::memory("media"))),
            )]),
            is_rtl: None,
            skip_entity_detection: None,
        };

        let mut copied = 0;
        message.copy_into(&mut |_| copied += 1);
        assert_eq!(copied, 2);

        let mut moved = 0;
        message.move_into(&mut |_| moved += 1);
        assert_eq!(moved, 2);
    }

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
