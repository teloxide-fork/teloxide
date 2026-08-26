use serde::{ser::SerializeStruct, Serialize};

use crate::types::{
    InputFile, InputFileLike, InputMediaAnimation, InputMediaAudio, InputMediaPhoto,
    InputMediaVideo, InputMediaVoiceNote, Location, RichBlockCaption, RichBlockTableCell, RichText,
};

/// A block in a rich formatted message to be sent.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    Paragraph(InputRichBlockParagraph),
    #[serde(rename = "heading")]
    Heading(InputRichBlockSectionHeading),
    #[serde(rename = "pre")]
    Pre(InputRichBlockPreformatted),
    Footer(InputRichBlockFooter),
    Divider(InputRichBlockDivider),
    MathematicalExpression(InputRichBlockMathematicalExpression),
    Anchor(InputRichBlockAnchor),
    List(InputRichBlockList),
    #[serde(rename = "blockquote")]
    BlockQuotation(InputRichBlockBlockQuotation),
    #[serde(rename = "pullquote")]
    PullQuotation(InputRichBlockPullQuotation),
    Collage(InputRichBlockCollage),
    Slideshow(InputRichBlockSlideshow),
    Table(InputRichBlockTable),
    Details(InputRichBlockDetails),
    Map(InputRichBlockMap),
    Animation(InputRichBlockAnimation),
    Audio(InputRichBlockAudio),
    Photo(InputRichBlockPhoto),
    Video(InputRichBlockVideo),
    VoiceNote(InputRichBlockVoiceNote),
    Thinking(InputRichBlockThinking),
}

macro_rules! text_block {
    ($name:ident) => {
        #[derive(Clone, Debug, Serialize)]
        #[cfg_attr(test, derive(schemars::JsonSchema))]
        pub struct $name {
            pub text: RichText,
        }
        impl $name {
            pub fn new(text: RichText) -> Self {
                Self { text }
            }
            pub fn text(mut self, text: RichText) -> Self {
                self.text = text;
                self
            }
        }
    };
}

/// An item of a rich-message list.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockListItem {
    pub blocks: Vec<InputRichBlock>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_checkbox: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_checked: bool,
    pub value: Option<i64>,
    /// The ordered-list label style, not the block discriminator.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

impl InputRichBlockListItem {
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self { blocks, has_checkbox: false, is_checked: false, value: None, r#type: None }
    }
    pub fn has_checkbox(mut self, value: bool) -> Self {
        self.has_checkbox = value;
        self
    }
    pub fn is_checked(mut self, value: bool) -> Self {
        self.is_checked = value;
        self
    }
    pub const fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }
}

text_block!(InputRichBlockParagraph);

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSectionHeading {
    pub text: RichText,
    pub size: i64,
}
impl InputRichBlockSectionHeading {
    pub fn new(text: RichText, size: i64) -> Self {
        Self { text, size }
    }
    pub fn text(mut self, text: RichText) -> Self {
        self.text = text;
        self
    }
    pub const fn size(mut self, size: i64) -> Self {
        self.size = size;
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPreformatted {
    pub text: RichText,
    pub language: Option<String>,
}
impl InputRichBlockPreformatted {
    pub fn new(text: RichText) -> Self {
        Self { text, language: None }
    }
    pub fn text(mut self, text: RichText) -> Self {
        self.text = text;
        self
    }
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

text_block!(InputRichBlockFooter);

#[derive(Clone, Copy, Debug, Default, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDivider {}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMathematicalExpression {
    pub expression: String,
}
impl InputRichBlockMathematicalExpression {
    pub fn new(expression: impl Into<String>) -> Self {
        Self { expression: expression.into() }
    }
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = expression.into();
        self
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnchor {
    pub name: String,
}
impl InputRichBlockAnchor {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockList {
    pub items: Vec<InputRichBlockListItem>,
}
impl InputRichBlockList {
    pub fn new(items: Vec<InputRichBlockListItem>) -> Self {
        Self { items }
    }
    pub fn items(mut self, items: Vec<InputRichBlockListItem>) -> Self {
        self.items = items;
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockBlockQuotation {
    pub blocks: Vec<InputRichBlock>,
    pub credit: Option<RichText>,
}
impl InputRichBlockBlockQuotation {
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self { blocks, credit: None }
    }
    pub fn blocks(mut self, blocks: Vec<InputRichBlock>) -> Self {
        self.blocks = blocks;
        self
    }
    pub fn credit(mut self, credit: RichText) -> Self {
        self.credit = Some(credit);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPullQuotation {
    pub text: RichText,
    pub credit: Option<RichText>,
}
impl InputRichBlockPullQuotation {
    pub fn new(text: RichText) -> Self {
        Self { text, credit: None }
    }
    pub fn text(mut self, text: RichText) -> Self {
        self.text = text;
        self
    }
    pub fn credit(mut self, credit: RichText) -> Self {
        self.credit = Some(credit);
        self
    }
}

macro_rules! block_collection {
    ($name:ident) => {
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Serialize)]
        #[cfg_attr(test, derive(schemars::JsonSchema))]
        pub struct $name {
            pub blocks: Vec<InputRichBlock>,
            pub caption: Option<RichBlockCaption>,
        }
        impl $name {
            pub fn new(blocks: Vec<InputRichBlock>) -> Self {
                Self { blocks, caption: None }
            }
            pub fn blocks(mut self, blocks: Vec<InputRichBlock>) -> Self {
                self.blocks = blocks;
                self
            }
            pub fn caption(mut self, caption: RichBlockCaption) -> Self {
                self.caption = Some(caption);
                self
            }
        }
    };
}
block_collection!(InputRichBlockCollage);
block_collection!(InputRichBlockSlideshow);

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockTable {
    pub cells: Vec<Vec<RichBlockTableCell>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_bordered: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_striped: bool,
    pub caption: Option<RichText>,
}
impl InputRichBlockTable {
    pub fn new(cells: Vec<Vec<RichBlockTableCell>>) -> Self {
        Self { cells, is_bordered: false, is_striped: false, caption: None }
    }
    pub fn cells(mut self, cells: Vec<Vec<RichBlockTableCell>>) -> Self {
        self.cells = cells;
        self
    }
    pub fn is_bordered(mut self, value: bool) -> Self {
        self.is_bordered = value;
        self
    }
    pub fn is_striped(mut self, value: bool) -> Self {
        self.is_striped = value;
        self
    }
    pub fn caption(mut self, caption: RichText) -> Self {
        self.caption = Some(caption);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDetails {
    pub summary: RichText,
    pub blocks: Vec<InputRichBlock>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_open: bool,
}
impl InputRichBlockDetails {
    pub fn new(summary: RichText, blocks: Vec<InputRichBlock>) -> Self {
        Self { summary, blocks, is_open: false }
    }
    pub fn summary(mut self, summary: RichText) -> Self {
        self.summary = summary;
        self
    }
    pub fn blocks(mut self, blocks: Vec<InputRichBlock>) -> Self {
        self.blocks = blocks;
        self
    }
    pub fn is_open(mut self, value: bool) -> Self {
        self.is_open = value;
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMap {
    pub location: Location,
    pub zoom: i64,
    pub width: i64,
    pub height: i64,
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockMap {
    pub const fn new(location: Location, zoom: i64, width: i64, height: i64) -> Self {
        Self { location, zoom, width, height, caption: None }
    }
    pub const fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }
    pub const fn zoom(mut self, zoom: i64) -> Self {
        self.zoom = zoom;
        self
    }
    pub const fn width(mut self, width: i64) -> Self {
        self.width = width;
        self
    }
    pub const fn height(mut self, height: i64) -> Self {
        self.height = height;
        self
    }
    pub fn caption(mut self, caption: RichBlockCaption) -> Self {
        self.caption = Some(caption);
        self
    }
}

macro_rules! media_block {
    ($name:ident, $field:ident, $media:ty, $media_type:literal) => {
        #[derive(Clone, Debug)]
        #[cfg_attr(test, derive(schemars::JsonSchema))]
        pub struct $name {
            pub $field: $media,
            pub caption: Option<RichBlockCaption>,
        }
        impl $name {
            pub fn new($field: $media) -> Self {
                Self { $field, caption: None }
            }
            pub fn $field(mut self, $field: $media) -> Self {
                self.$field = $field;
                self
            }
            pub fn caption(mut self, caption: RichBlockCaption) -> Self {
                self.caption = Some(caption);
                self
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #[derive(Serialize)]
                struct TypedMedia<'a, T: Serialize> {
                    #[serde(rename = "type")]
                    r#type: &'static str,
                    #[serde(flatten)]
                    media: &'a T,
                }

                let mut state = serializer.serialize_struct(stringify!($name), 2)?;
                state.serialize_field(
                    stringify!($field),
                    &TypedMedia { r#type: $media_type, media: &self.$field },
                )?;
                if let Some(caption) = &self.caption {
                    state.serialize_field("caption", caption)?;
                }
                state.end()
            }
        }
    };
}
media_block!(InputRichBlockAnimation, animation, InputMediaAnimation, "animation");
media_block!(InputRichBlockAudio, audio, InputMediaAudio, "audio");
media_block!(InputRichBlockPhoto, photo, InputMediaPhoto, "photo");
media_block!(InputRichBlockVideo, video, InputMediaVideo, "video");
media_block!(InputRichBlockVoiceNote, voice_note, InputMediaVoiceNote, "voice_note");
text_block!(InputRichBlockThinking);

impl InputFileLike for InputRichBlock {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::List(list) => list
                .items
                .iter()
                .flat_map(|item| &item.blocks)
                .for_each(|block| block.copy_into(into)),
            Self::BlockQuotation(block) => {
                block.blocks.iter().for_each(|block| block.copy_into(into));
            }
            Self::Collage(block) => {
                block.blocks.iter().for_each(|block| block.copy_into(into));
            }
            Self::Slideshow(block) => {
                block.blocks.iter().for_each(|block| block.copy_into(into));
            }
            Self::Details(block) => block.blocks.iter().for_each(|block| block.copy_into(into)),
            Self::Animation(block) => {
                block.animation.media.copy_into(into);
                block.animation.thumbnail.copy_into(into);
            }
            Self::Audio(block) => {
                block.audio.media.copy_into(into);
                block.audio.thumbnail.copy_into(into);
            }
            Self::Photo(block) => block.photo.media.copy_into(into),
            Self::Video(block) => {
                block.video.media.copy_into(into);
                block.video.thumbnail.copy_into(into);
                block.video.cover.copy_into(into);
            }
            Self::VoiceNote(block) => block.voice_note.media.copy_into(into),
            Self::Paragraph(_)
            | Self::Heading(_)
            | Self::Pre(_)
            | Self::Footer(_)
            | Self::Divider(_)
            | Self::MathematicalExpression(_)
            | Self::Anchor(_)
            | Self::PullQuotation(_)
            | Self::Table(_)
            | Self::Map(_)
            | Self::Thinking(_) => {}
        }
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        match self {
            Self::List(list) => list
                .items
                .iter_mut()
                .flat_map(|item| &mut item.blocks)
                .for_each(|block| block.move_into(into)),
            Self::BlockQuotation(block) => {
                block.blocks.iter_mut().for_each(|block| block.move_into(into));
            }
            Self::Collage(block) => {
                block.blocks.iter_mut().for_each(|block| block.move_into(into));
            }
            Self::Slideshow(block) => {
                block.blocks.iter_mut().for_each(|block| block.move_into(into));
            }
            Self::Details(block) => block.blocks.iter_mut().for_each(|block| block.move_into(into)),
            Self::Animation(block) => {
                block.animation.media.move_into(into);
                block.animation.thumbnail.move_into(into);
            }
            Self::Audio(block) => {
                block.audio.media.move_into(into);
                block.audio.thumbnail.move_into(into);
            }
            Self::Photo(block) => block.photo.media.move_into(into),
            Self::Video(block) => {
                block.video.media.move_into(into);
                block.video.thumbnail.move_into(into);
                block.video.cover.move_into(into);
            }
            Self::VoiceNote(block) => block.voice_note.media.move_into(into),
            Self::Paragraph(_)
            | Self::Heading(_)
            | Self::Pre(_)
            | Self::Footer(_)
            | Self::Divider(_)
            | Self::MathematicalExpression(_)
            | Self::Anchor(_)
            | Self::PullQuotation(_)
            | Self::Table(_)
            | Self::Map(_)
            | Self::Thinking(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{InputFile, InputMedia, InputMediaVoiceNote};

    fn text(text: &str) -> RichText {
        RichText::Plain(text.into())
    }

    #[test]
    fn serializes_every_discriminator() {
        let text = || text("x");
        let blocks = vec![
            (InputRichBlock::Paragraph(InputRichBlockParagraph::new(text())), "paragraph"),
            (InputRichBlock::Heading(InputRichBlockSectionHeading::new(text(), 1)), "heading"),
            (InputRichBlock::Pre(InputRichBlockPreformatted::new(text())), "pre"),
            (InputRichBlock::Footer(InputRichBlockFooter::new(text())), "footer"),
            (InputRichBlock::Divider(InputRichBlockDivider {}), "divider"),
            (
                InputRichBlock::MathematicalExpression(InputRichBlockMathematicalExpression::new(
                    "x",
                )),
                "mathematical_expression",
            ),
            (InputRichBlock::Anchor(InputRichBlockAnchor::new("x")), "anchor"),
            (InputRichBlock::List(InputRichBlockList::new(vec![])), "list"),
            (
                InputRichBlock::BlockQuotation(InputRichBlockBlockQuotation::new(vec![])),
                "blockquote",
            ),
            (InputRichBlock::PullQuotation(InputRichBlockPullQuotation::new(text())), "pullquote"),
            (InputRichBlock::Collage(InputRichBlockCollage::new(vec![])), "collage"),
            (InputRichBlock::Slideshow(InputRichBlockSlideshow::new(vec![])), "slideshow"),
            (InputRichBlock::Table(InputRichBlockTable::new(vec![])), "table"),
            (InputRichBlock::Details(InputRichBlockDetails::new(text(), vec![])), "details"),
            (
                InputRichBlock::Map(InputRichBlockMap::new(
                    Location {
                        longitude: 1.0,
                        latitude: 2.0,
                        horizontal_accuracy: None,
                        live_period: None,
                        heading: None,
                        proximity_alert_radius: None,
                    },
                    13,
                    1,
                    1,
                )),
                "map",
            ),
            (
                InputRichBlock::Animation(InputRichBlockAnimation::new(InputMediaAnimation::new(
                    InputFile::file_id("a".into()),
                ))),
                "animation",
            ),
            (
                InputRichBlock::Audio(InputRichBlockAudio::new(InputMediaAudio::new(
                    InputFile::file_id("a".into()),
                ))),
                "audio",
            ),
            (
                InputRichBlock::Photo(InputRichBlockPhoto::new(InputMediaPhoto::new(
                    InputFile::file_id("a".into()),
                ))),
                "photo",
            ),
            (
                InputRichBlock::Video(InputRichBlockVideo::new(InputMediaVideo::new(
                    InputFile::file_id("a".into()),
                ))),
                "video",
            ),
            (
                InputRichBlock::VoiceNote(InputRichBlockVoiceNote::new(InputMediaVoiceNote::new(
                    InputFile::file_id("a".into()),
                ))),
                "voice_note",
            ),
            (InputRichBlock::Thinking(InputRichBlockThinking::new(text())), "thinking"),
        ];
        for (block, ty) in blocks {
            assert_eq!(serde_json::to_value(block).unwrap()["type"], ty);
        }
    }

    #[test]
    fn serializes_nested_list_and_list_style_type() {
        let block =
            InputRichBlock::List(InputRichBlockList::new(vec![InputRichBlockListItem::new(vec![
                InputRichBlock::Paragraph(InputRichBlockParagraph::new(text("item"))),
            ])
            .r#type("a")]));
        let value = serde_json::to_value(block).unwrap();
        assert_eq!(value["items"][0]["type"], "a");
        assert_eq!(value["items"][0]["blocks"][0]["type"], "paragraph");
        assert!(value["items"][0].get("label").is_none());
    }

    #[test]
    fn serializes_rich_text_captions_and_typed_media_fields() {
        let caption = RichBlockCaption { text: text("caption"), credit: Some(text("credit")) };
        let animation = InputRichBlock::Animation(
            InputRichBlockAnimation::new(InputMediaAnimation::new(InputFile::file_id("id".into())))
                .caption(caption),
        );

        let value = serde_json::to_value(animation).unwrap();
        assert_eq!(value["animation"]["type"], "animation");
        assert!(value.get("media").is_none());
        assert_eq!(value["caption"]["text"], "caption");
        assert_eq!(value["caption"]["credit"], "credit");
    }

    #[test]
    fn serializes_voice_note_media() {
        let value = serde_json::to_value(InputMedia::VoiceNote(InputMediaVoiceNote::new(
            InputFile::file_id("voice".into()),
        )))
        .unwrap();
        assert_eq!(value["type"], "voice_note");
        assert_eq!(value["media"], "voice");
    }
}
