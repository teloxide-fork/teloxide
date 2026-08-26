use serde::{Deserialize, Serialize};

use crate::types::{Animation, Audio, Location, PhotoSize, RichText, Video, Voice};

/// A block in a rich formatted message.
///
/// It is always an object (never a `String`/`Array`, unlike [`RichText`]).
///
/// [The official docs](https://core.telegram.org/bots/api#richblock).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlock {
    Paragraph(RichBlockParagraph),
    #[serde(rename = "heading")]
    Heading(RichBlockSectionHeading),
    #[serde(rename = "pre")]
    Pre(RichBlockPreformatted),
    Footer(RichBlockFooter),
    Divider(RichBlockDivider),
    MathematicalExpression(RichBlockMathematicalExpression),
    Anchor(RichBlockAnchor),
    List(RichBlockList),
    #[serde(rename = "blockquote")]
    Blockquote(RichBlockBlockQuotation),
    #[serde(rename = "pullquote")]
    Pullquote(RichBlockPullQuotation),
    Collage(RichBlockCollage),
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Details(RichBlockDetails),
    Map(RichBlockMap),
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    Photo(RichBlockPhoto),
    Video(RichBlockVideo),
    VoiceNote(RichBlockVoiceNote),
    Thinking(RichBlockThinking),
}

/// Horizontal alignment of a [`RichBlockTableCell`].
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktablecell).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum CellAlign {
    Left,
    Center,
    Right,
}

/// Vertical alignment of a [`RichBlockTableCell`].
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktablecell).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum CellVAlign {
    Top,
    Middle,
    Bottom,
}

/// Caption of a rich formatted block.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcaption).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCaption {
    /// Block caption.
    pub text: RichText,

    /// Block credit, corresponds to HTML `<cite>`.
    pub credit: Option<RichText>,
}

/// A cell in a table.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktablecell).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTableCell {
    /// Text in the cell. If omitted, the cell is invisible.
    pub text: Option<RichText>,

    /// `true` if the cell is a header cell.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_header: bool,

    /// Number of columns the cell spans if `> 1`.
    pub colspan: Option<i64>,

    /// Number of rows the cell spans if `> 1`.
    pub rowspan: Option<i64>,

    /// Horizontal alignment of the cell.
    pub align: CellAlign,

    /// Vertical alignment of the cell.
    pub valign: CellVAlign,
}

/// An item of a list.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocklistitem).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockListItem {
    /// Label of the item.
    pub label: String,

    /// The content of the item.
    pub blocks: Vec<RichBlock>,

    /// `true` if the item has a checkbox.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_checkbox: bool,

    /// `true` if the item has a checked checkbox.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_checked: bool,

    /// For ordered lists, numeric value of the item label.
    pub value: Option<i64>,

    /// For ordered lists, type of item label: `"a"`, `"A"`, `"i"`, `"I"`, or
    /// `"1"`.
    ///
    /// This is a data field (ordered-list label style), **not** a serde
    /// discriminator.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// Corresponds to HTML `<p>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockparagraph).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockParagraph {
    /// Text of the block.
    pub text: RichText,
}

/// Corresponds to HTML `<h1>`-`<h6>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocksectionheading).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSectionHeading {
    /// Text of the block.
    pub text: RichText,

    /// Relative font size; 1-6, 1 is largest.
    pub size: i64,
}

/// Corresponds to nested `<pre>`/`<code>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockpreformatted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,

    /// Programming language of the text.
    pub language: Option<String>,
}

/// Corresponds to HTML `<footer>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockfooter).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockFooter {
    /// Text of the block.
    pub text: RichText,
}

/// Corresponds to HTML `<hr/>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdivider).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDivider {}

/// Corresponds to `<tg-math-block>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmathematicalexpression).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format.
    pub expression: String,
}

/// Corresponds to `<a name="...">`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanchor).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnchor {
    /// The name of the anchor.
    pub name: String,
}

/// Corresponds to `<ul>`/`<ol>` with `<li>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocklist).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockList {
    /// Items of the list.
    pub items: Vec<RichBlockListItem>,
}

/// Corresponds to `<blockquote>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockblockquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<RichBlock>,

    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// Loosely corresponds to `<aside>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockpullquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,

    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// Corresponds to `<tg-collage>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcollage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<RichBlock>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<tg-slideshow>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockslideshow).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<RichBlock>,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<table>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktable).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTable {
    /// Cells of the table (rows of columns).
    pub cells: Vec<Vec<RichBlockTableCell>>,

    /// `true` if the table has borders.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_bordered: bool,

    /// `true` if the table is striped.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_striped: bool,

    /// Caption of the table.
    ///
    /// Note: unlike every other caption-bearing block, this is a plain
    /// [`RichText`], not a [`RichBlockCaption`].
    pub caption: Option<RichText>,
}

/// Corresponds to `<details>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdetails).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDetails {
    /// Always-shown summary of the block.
    pub summary: RichText,

    /// Content of the block.
    pub blocks: Vec<RichBlock>,

    /// `true` if content is visible by default.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_open: bool,
}

/// Corresponds to `<tg-map>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmap).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMap {
    /// Location of the center of the map.
    pub location: Location,

    /// Map zoom level; 13-20.
    pub zoom: i64,

    /// Expected width of the map.
    pub width: i64,

    /// Expected height of the map.
    pub height: i64,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnimation {
    /// The animation.
    pub animation: Animation,

    /// `true` if preview covered by spoiler.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAudio {
    /// The audio.
    pub audio: Audio,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<photo>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPhoto {
    /// Available sizes of the photo.
    pub photo: Vec<PhotoSize>,

    /// `true` if preview covered by spoiler.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVideo {
    /// The video.
    pub video: Video,

    /// `true` if preview covered by spoiler.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvoicenote).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVoiceNote {
    /// The voice note.
    pub voice_note: Voice,

    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// Corresponds to custom `<tg-thinking>`.
///
/// This block is send-only — it may be used only in `sendRichMessageDraft`
/// and can't be received in messages.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockthinking).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockThinking {
    /// Text of the block.
    pub text: RichText,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FileMeta;

    fn plain(s: &str) -> RichText {
        RichText::Plain(s.to_owned())
    }

    #[test]
    fn deserialize_list_with_nested_items() {
        let json = r#"{
            "type":"list",
            "items":[
                {
                    "label":"1.",
                    "blocks":[{"type":"paragraph","text":"hello"}]
                }
            ]
        }"#;
        let block: RichBlock = serde_json::from_str(json).unwrap();

        assert_eq!(
            block,
            RichBlock::List(RichBlockList {
                items: vec![RichBlockListItem {
                    label: "1.".to_owned(),
                    blocks: vec![RichBlock::Paragraph(RichBlockParagraph { text: plain("hello") })],
                    has_checkbox: false,
                    is_checked: false,
                    value: None,
                    r#type: None,
                }],
            })
        );
    }

    #[test]
    fn deserialize_table_with_2d_cells() {
        let json = r#"{
            "type":"table",
            "cells":[
                [
                    {"text":"a","align":"left","valign":"top"},
                    {"text":"b","align":"center","valign":"middle"}
                ],
                [
                    {"text":"c","align":"right","valign":"bottom"},
                    {"align":"left","valign":"top"}
                ]
            ],
            "caption":"a table"
        }"#;
        let block: RichBlock = serde_json::from_str(json).unwrap();

        let RichBlock::Table(table) = block else {
            panic!("expected RichBlock::Table");
        };

        assert_eq!(table.cells.len(), 2);
        assert_eq!(table.cells[0].len(), 2);
        assert_eq!(table.cells[0][0].align, CellAlign::Left);
        assert_eq!(table.cells[0][0].valign, CellVAlign::Top);
        assert_eq!(table.cells[0][1].align, CellAlign::Center);
        assert_eq!(table.cells[0][1].valign, CellVAlign::Middle);
        assert_eq!(table.cells[1][0].align, CellAlign::Right);
        assert_eq!(table.cells[1][0].valign, CellVAlign::Bottom);
        assert_eq!(table.cells[1][1].text, None);

        // `caption` must be a plain `RichText`, not a `RichBlockCaption`.
        let caption: RichText = table.caption.unwrap();
        assert_eq!(caption, plain("a table"));
    }

    #[test]
    fn deserialize_blockquote_with_nested_blocks() {
        let json = r#"{
            "type":"blockquote",
            "blocks":[{"type":"paragraph","text":"quoted"}],
            "credit":"someone"
        }"#;
        let block: RichBlock = serde_json::from_str(json).unwrap();

        assert_eq!(
            block,
            RichBlock::Blockquote(RichBlockBlockQuotation {
                blocks: vec![RichBlock::Paragraph(RichBlockParagraph { text: plain("quoted") })],
                credit: Some(plain("someone")),
            })
        );
    }

    #[test]
    fn deserialize_photo_media_block() {
        let json = r#"{
            "type":"photo",
            "photo":[{"file_id":"id","file_unique_id":"uid","width":100,"height":100,"file_size":123}],
            "has_spoiler":true
        }"#;
        let block: RichBlock = serde_json::from_str(json).unwrap();

        assert_eq!(
            block,
            RichBlock::Photo(RichBlockPhoto {
                photo: vec![PhotoSize {
                    file: FileMeta {
                        id: crate::types::FileId("id".to_owned()),
                        unique_id: crate::types::FileUniqueId("uid".to_owned()),
                        size: 123,
                    },
                    width: 100,
                    height: 100,
                }],
                has_spoiler: true,
                caption: None,
            })
        );
    }

    #[test]
    fn list_item_type_field_round_trips() {
        let json = r#"{"label":"a.","blocks":[],"type":"a"}"#;
        let item: RichBlockListItem = serde_json::from_str(json).unwrap();

        assert_eq!(item.r#type, Some("a".to_owned()));

        let serialized = serde_json::to_value(&item).unwrap();
        assert_eq!(serialized["type"], "a");
    }

    #[test]
    fn rich_block_variants_serialize() {
        let text = || plain("x");

        let blocks: Vec<(RichBlock, &str)> = vec![
            (RichBlock::Paragraph(RichBlockParagraph { text: text() }), "paragraph"),
            (RichBlock::Heading(RichBlockSectionHeading { text: text(), size: 1 }), "heading"),
            (RichBlock::Pre(RichBlockPreformatted { text: text(), language: None }), "pre"),
            (RichBlock::Footer(RichBlockFooter { text: text() }), "footer"),
            (RichBlock::Divider(RichBlockDivider {}), "divider"),
            (
                RichBlock::MathematicalExpression(RichBlockMathematicalExpression {
                    expression: "x".to_owned(),
                }),
                "mathematical_expression",
            ),
            (RichBlock::Anchor(RichBlockAnchor { name: "n".to_owned() }), "anchor"),
            (RichBlock::List(RichBlockList { items: vec![] }), "list"),
            (
                RichBlock::Blockquote(RichBlockBlockQuotation { blocks: vec![], credit: None }),
                "blockquote",
            ),
            (
                RichBlock::Pullquote(RichBlockPullQuotation { text: text(), credit: None }),
                "pullquote",
            ),
            (RichBlock::Collage(RichBlockCollage { blocks: vec![], caption: None }), "collage"),
            (
                RichBlock::Slideshow(RichBlockSlideshow { blocks: vec![], caption: None }),
                "slideshow",
            ),
            (
                RichBlock::Table(RichBlockTable {
                    cells: vec![],
                    is_bordered: false,
                    is_striped: false,
                    caption: None,
                }),
                "table",
            ),
            (
                RichBlock::Details(RichBlockDetails {
                    summary: text(),
                    blocks: vec![],
                    is_open: false,
                }),
                "details",
            ),
            (
                RichBlock::Map(RichBlockMap {
                    location: Location {
                        longitude: 1.0,
                        latitude: 2.0,
                        horizontal_accuracy: None,
                        live_period: None,
                        heading: None,
                        proximity_alert_radius: None,
                    },
                    zoom: 15,
                    width: 100,
                    height: 100,
                    caption: None,
                }),
                "map",
            ),
            (
                RichBlock::Animation(RichBlockAnimation {
                    animation: Animation {
                        file: FileMeta {
                            id: crate::types::FileId("id".to_owned()),
                            unique_id: crate::types::FileUniqueId("uid".to_owned()),
                            size: 0,
                        },
                        width: 1,
                        height: 1,
                        duration: crate::types::Seconds::from_seconds(1),
                        thumbnail: None,
                        file_name: None,
                        mime_type: None,
                    },
                    has_spoiler: false,
                    caption: None,
                }),
                "animation",
            ),
            (
                RichBlock::Audio(RichBlockAudio {
                    audio: Audio {
                        file: FileMeta {
                            id: crate::types::FileId("id".to_owned()),
                            unique_id: crate::types::FileUniqueId("uid".to_owned()),
                            size: 0,
                        },
                        duration: crate::types::Seconds::from_seconds(1),
                        performer: None,
                        title: None,
                        file_name: None,
                        mime_type: None,
                        thumbnail: None,
                    },
                    caption: None,
                }),
                "audio",
            ),
            (
                RichBlock::Photo(RichBlockPhoto {
                    photo: vec![],
                    has_spoiler: false,
                    caption: None,
                }),
                "photo",
            ),
            (
                RichBlock::Video(RichBlockVideo {
                    video: Video {
                        file: FileMeta {
                            id: crate::types::FileId("id".to_owned()),
                            unique_id: crate::types::FileUniqueId("uid".to_owned()),
                            size: 0,
                        },
                        width: 1,
                        height: 1,
                        duration: crate::types::Seconds::from_seconds(1),
                        thumbnail: None,
                        cover: None,
                        start_timestamp: None,
                        qualities: None,
                        file_name: None,
                        mime_type: None,
                    },
                    has_spoiler: false,
                    caption: None,
                }),
                "video",
            ),
            (
                RichBlock::VoiceNote(RichBlockVoiceNote {
                    voice_note: Voice {
                        file: FileMeta {
                            id: crate::types::FileId("id".to_owned()),
                            unique_id: crate::types::FileUniqueId("uid".to_owned()),
                            size: 0,
                        },
                        duration: crate::types::Seconds::from_seconds(1),
                        mime_type: None,
                    },
                    caption: None,
                }),
                "voice_note",
            ),
            (RichBlock::Thinking(RichBlockThinking { text: text() }), "thinking"),
        ];

        assert_eq!(blocks.len(), 21);

        for (block, wire_type) in blocks {
            assert_eq!(serde_json::to_value(block).unwrap()["type"], wire_type);
        }
    }
}
