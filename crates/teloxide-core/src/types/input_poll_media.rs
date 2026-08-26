use serde::Serialize;

use crate::types::{
    InputFile, InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaLivePhoto,
    InputMediaLocation, InputMediaPhoto, InputMediaSticker, InputMediaVenue, InputMediaVideo,
};

/// This object represents media to be sent with a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollMedia {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

/// This object represents media to be sent with a poll option.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloptionmedia).
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPollOptionMedia {
    Animation(InputMediaAnimation),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Sticker(InputMediaSticker),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

impl InputPollMedia {
    /// Returns all files contained in this poll media.
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputPollMedia::*;

        let files = match self {
            Animation(media) => [Some(&media.media), media.thumbnail.as_ref(), None],
            Audio(media) => [Some(&media.media), media.thumbnail.as_ref(), None],
            Document(media) => [Some(&media.media), media.thumbnail.as_ref(), None],
            LivePhoto(media) => [Some(&media.media), Some(&media.photo), None],
            Location(_) | Venue(_) => [None, None, None],
            Photo(media) => [Some(&media.media), None, None],
            Video(media) => [Some(&media.media), media.thumbnail.as_ref(), media.cover.as_ref()],
        };

        files.into_iter().flatten()
    }

    /// Returns all files contained in this poll media.
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputPollMedia::*;

        let files = match self {
            Animation(media) => [Some(&mut media.media), media.thumbnail.as_mut(), None],
            Audio(media) => [Some(&mut media.media), media.thumbnail.as_mut(), None],
            Document(media) => [Some(&mut media.media), media.thumbnail.as_mut(), None],
            LivePhoto(media) => [Some(&mut media.media), Some(&mut media.photo), None],
            Location(_) | Venue(_) => [None, None, None],
            Photo(media) => [Some(&mut media.media), None, None],
            Video(media) => {
                [Some(&mut media.media), media.thumbnail.as_mut(), media.cover.as_mut()]
            }
        };

        files.into_iter().flatten()
    }
}

impl InputPollOptionMedia {
    /// Returns all files contained in this poll option media.
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputPollOptionMedia::*;

        let files = match self {
            Animation(media) => [Some(&media.media), media.thumbnail.as_ref(), None],
            LivePhoto(media) => [Some(&media.media), Some(&media.photo), None],
            Location(_) | Venue(_) => [None, None, None],
            Photo(media) => [Some(&media.media), None, None],
            Sticker(media) => [Some(&media.media), None, None],
            Video(media) => [Some(&media.media), media.thumbnail.as_ref(), media.cover.as_ref()],
        };

        files.into_iter().flatten()
    }

    /// Returns all files contained in this poll option media.
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputPollOptionMedia::*;

        let files = match self {
            Animation(media) => [Some(&mut media.media), media.thumbnail.as_mut(), None],
            LivePhoto(media) => [Some(&mut media.media), Some(&mut media.photo), None],
            Location(_) | Venue(_) => [None, None, None],
            Photo(media) => [Some(&mut media.media), None, None],
            Sticker(media) => [Some(&mut media.media), None, None],
            Video(media) => {
                [Some(&mut media.media), media.thumbnail.as_mut(), media.cover.as_mut()]
            }
        };

        files.into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        InputFile, InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaLivePhoto,
        InputMediaLocation, InputMediaPhoto, InputMediaSticker, InputMediaVenue, InputMediaVideo,
    };

    use super::*;

    #[test]
    fn poll_media_variants_serialize() {
        let file = || InputFile::file_id("media".into());
        let media = vec![
            (InputPollMedia::Animation(InputMediaAnimation::new(file())), "animation"),
            (InputPollMedia::Audio(InputMediaAudio::new(file())), "audio"),
            (InputPollMedia::Document(InputMediaDocument::new(file())), "document"),
            (InputPollMedia::LivePhoto(InputMediaLivePhoto::new(file(), file())), "live_photo"),
            (InputPollMedia::Location(InputMediaLocation::new(1.0, 2.0)), "location"),
            (InputPollMedia::Photo(InputMediaPhoto::new(file())), "photo"),
            (InputPollMedia::Venue(InputMediaVenue::new(1.0, 2.0, "Venue", "Address")), "venue"),
            (InputPollMedia::Video(InputMediaVideo::new(file())), "video"),
        ];

        for (media, kind) in media {
            assert_eq!(serde_json::to_value(media).unwrap()["type"], kind);
        }
    }

    #[test]
    fn poll_option_media_variants_serialize() {
        let file = || InputFile::file_id("media".into());
        let media = vec![
            (InputPollOptionMedia::Animation(InputMediaAnimation::new(file())), "animation"),
            (
                InputPollOptionMedia::LivePhoto(InputMediaLivePhoto::new(file(), file())),
                "live_photo",
            ),
            (InputPollOptionMedia::Location(InputMediaLocation::new(1.0, 2.0)), "location"),
            (InputPollOptionMedia::Photo(InputMediaPhoto::new(file())), "photo"),
            (InputPollOptionMedia::Sticker(InputMediaSticker::new(file())), "sticker"),
            (
                InputPollOptionMedia::Venue(InputMediaVenue::new(1.0, 2.0, "Venue", "Address")),
                "venue",
            ),
            (InputPollOptionMedia::Video(InputMediaVideo::new(file())), "video"),
        ];

        for (media, kind) in media {
            assert_eq!(serde_json::to_value(media).unwrap()["type"], kind);
        }
    }
}
