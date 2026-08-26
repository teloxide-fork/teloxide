use serde::Serialize;

use crate::types::InputRichMedia;

/// Media available to blocks in an
/// [`InputRichMessage`](crate::types::InputRichMessage).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageMedia {
    /// Identifier referenced by rich-message media blocks.
    pub id: String,
    /// Media to upload or reuse.
    pub media: InputRichMedia,
}

impl InputRichMessageMedia {
    pub fn new(id: impl Into<String>, media: InputRichMedia) -> Self {
        Self { id: id.into(), media }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn media(mut self, media: InputRichMedia) -> Self {
        self.media = media;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        InputFile, InputMediaPhoto, InputMediaVoiceNote, InputRichMedia, InputRichMessageMedia,
    };

    #[test]
    fn serializes_media_reference() {
        let media = InputRichMessageMedia::new(
            "photo",
            InputRichMedia::Photo(InputMediaPhoto::new(InputFile::file_id("id".into()))),
        );

        assert_eq!(
            serde_json::to_value(media).unwrap(),
            serde_json::json!({"id": "photo", "media": {"type": "photo", "media": "id"}})
        );
    }

    #[test]
    fn serializes_voice_note_rich_media() {
        let value = serde_json::to_value(InputRichMessageMedia::new(
            "voice",
            InputRichMedia::VoiceNote(InputMediaVoiceNote::new(InputFile::file_id("id".into()))),
        ))
        .unwrap();

        assert_eq!(value["media"]["type"], "voice_note");
    }

    #[test]
    fn schema_exposes_only_rich_message_media_variants() {
        let schema = serde_json::to_value(schemars::schema_for!(InputRichMessageMedia)).unwrap();
        let variants = schema["properties"]["media"]["anyOf"].as_array().unwrap();

        assert_eq!(variants.len(), 5);
        for variant in [
            "InputMediaAnimation",
            "InputMediaAudio",
            "InputMediaPhoto",
            "InputMediaVideo",
            "InputMediaVoiceNote",
        ] {
            assert!(variants.iter().any(|value| value["$ref"] == format!("#/$defs/{variant}")));
        }
    }
}
