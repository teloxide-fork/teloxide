use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, LivePhoto, Location, PhotoSize, Sticker, Venue, Video,
};

/// This object describes media attached to a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#pollmedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    /// Animation attached to the poll.
    pub animation: Option<Animation>,

    /// Audio attached to the poll.
    pub audio: Option<Audio>,

    /// Document attached to the poll.
    pub document: Option<Document>,

    /// Live photo attached to the poll.
    pub live_photo: Option<LivePhoto>,

    /// Location attached to the poll.
    pub location: Option<Location>,

    /// Photo attached to the poll.
    pub photo: Option<Vec<PhotoSize>>,

    /// Sticker attached to the poll.
    pub sticker: Option<Sticker>,

    /// Venue attached to the poll.
    pub venue: Option<Venue>,

    /// Video attached to the poll.
    pub video: Option<Video>,
}

#[cfg(test)]
mod tests {
    use crate::types::Seconds;

    use super::*;

    #[test]
    fn deserialize_flat_object() {
        let media: PollMedia = serde_json::from_str(
            r#"{
                "live_photo":{
                    "file_id":"live-photo",
                    "file_unique_id":"unique-live-photo",
                    "width":320,
                    "height":240,
                    "duration":3
                }
            }"#,
        )
        .unwrap();

        assert_eq!(media.live_photo.unwrap().duration, Seconds::from_seconds(3));
    }

    #[test]
    fn serialize_flat_object() {
        let media = PollMedia {
            animation: None,
            audio: None,
            document: None,
            live_photo: Some(LivePhoto {
                file_id: "live-photo".to_owned(),
                file_unique_id: "unique-live-photo".to_owned(),
                width: 320,
                height: 240,
                duration: Seconds::from_seconds(3),
                photo: None,
                mime_type: None,
                file_size: None,
            }),
            location: None,
            photo: None,
            sticker: None,
            venue: None,
            video: None,
        };

        let value = serde_json::to_value(media).unwrap();
        assert_eq!(value["live_photo"]["file_id"], "live-photo");
        assert!(value.get("type").is_none());
    }
}
