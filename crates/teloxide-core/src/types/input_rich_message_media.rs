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
        InputFile, InputMediaDocument, InputMediaPhoto, InputMediaVoiceNote, InputRichMedia,
        InputRichMessageMedia,
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
    fn document_rich_media_preserves_the_public_union_field_and_traverses_its_files() {
        let document = InputFile::memory("document");
        let thumbnail = InputFile::memory("thumbnail");
        let document_wire = serde_json::to_value(&document).unwrap();
        let thumbnail_wire = serde_json::to_value(&thumbnail).unwrap();
        let mut media = InputRichMessageMedia::new(
            "document",
            InputRichMedia::Document(InputMediaDocument::new(document).thumbnail(thumbnail)),
        );
        let mut files = vec![];

        let _: InputRichMedia = media.media.clone();

        let value = serde_json::to_value(&media).unwrap();
        assert_eq!(value["media"]["type"], "document");
        assert_eq!(value["media"]["media"], document_wire);
        assert_eq!(value["media"]["thumbnail"], thumbnail_wire);

        media.media.copy_into(&mut |file| files.push(file));
        assert_eq!(files.len(), 2);
        assert_eq!(serde_json::to_value(&files[0]).unwrap(), document_wire);
        assert_eq!(serde_json::to_value(&files[1]).unwrap(), thumbnail_wire);

        media.media.move_into(&mut |file| files.push(file));
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn schema_exposes_all_rich_message_media_variants() {
        let schema = serde_json::to_value(schemars::schema_for!(InputRichMessageMedia)).unwrap();
        let variants = schema["properties"]["media"]["anyOf"].as_array().unwrap();

        assert_eq!(variants.len(), 6);
        for variant in [
            "InputMediaAnimation",
            "InputMediaAudio",
            "InputMediaDocument",
            "InputMediaPhoto",
            "InputMediaVideo",
            "InputMediaVoiceNote",
        ] {
            assert!(variants.iter().any(|value| value["$ref"] == format!("#/$defs/{variant}")));
        }
    }
}
