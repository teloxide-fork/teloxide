use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Seconds};

/// This object represents a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Identifier for this file, which can be used to download or reuse the
    /// file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots.
    pub file_unique_id: String,

    /// Live photo width as defined by sender.
    pub width: u32,

    /// Live photo height as defined by sender.
    pub height: u32,

    /// Duration of the live photo in seconds as defined by sender.
    pub duration: Seconds,

    /// Available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// MIME type of the file as defined by sender.
    #[serde(default, with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<String>"))]
    pub mime_type: Option<Mime>,

    /// File size in bytes.
    pub file_size: Option<u32>,
}

#[cfg(test)]
mod tests {
    use mime::Mime;

    use crate::types::LivePhoto;

    #[test]
    fn deserialize() {
        let live_photo: LivePhoto = serde_json::from_str(
            r#"{
                "file_id":"live-photo",
                "file_unique_id":"unique-live-photo",
                "width":320,
                "height":240,
                "duration":3,
                "photo":[],
                "mime_type":"image/jpeg",
                "file_size":1234
            }"#,
        )
        .unwrap();

        assert_eq!(live_photo.file_id, "live-photo");
        assert_eq!(live_photo.file_unique_id, "unique-live-photo");
        assert_eq!(live_photo.width, 320);
        assert_eq!(live_photo.height, 240);
        assert_eq!(live_photo.duration.seconds(), 3);
        assert_eq!(live_photo.photo, Some(vec![]));
        assert_eq!(live_photo.mime_type.unwrap(), "image/jpeg".parse::<Mime>().unwrap());
        assert_eq!(live_photo.file_size, Some(1234));
    }

    #[test]
    fn deserialize_without_mime_type() {
        let live_photo: LivePhoto = serde_json::from_str(
            r#"{
                "file_id":"live-photo",
                "file_unique_id":"unique-live-photo",
                "width":320,
                "height":240,
                "duration":3
            }"#,
        )
        .unwrap();

        assert_eq!(live_photo.mime_type, None);
    }
}
