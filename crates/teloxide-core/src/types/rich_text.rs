use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, User};

/// Rich formatted text.
///
/// It can be either a `String` for plain text, an array of `RichText`
/// objects, or one of the named formatting objects.
///
/// [The official docs](https://core.telegram.org/bots/api#richtext).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RichText {
    Plain(String),
    List(Vec<RichText>),
    Formatted(Box<RichTextKind>),
}

/// The kind of a formatted [`RichText`] value.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextKind {
    Bold(RichTextBold),
    Italic(RichTextItalic),
    Underline(RichTextUnderline),
    Strikethrough(RichTextStrikethrough),
    Spoiler(RichTextSpoiler),
    DateTime(RichTextDateTime),
    TextMention(RichTextTextMention),
    Subscript(RichTextSubscript),
    Superscript(RichTextSuperscript),
    Marked(RichTextMarked),
    Code(RichTextCode),
    CustomEmoji(RichTextCustomEmoji),
    MathematicalExpression(RichTextMathematicalExpression),
    Url(RichTextUrl),
    EmailAddress(RichTextEmailAddress),
    PhoneNumber(RichTextPhoneNumber),
    BankCardNumber(RichTextBankCardNumber),
    Mention(RichTextMention),
    Hashtag(RichTextHashtag),
    Cashtag(RichTextCashtag),
    BotCommand(RichTextBotCommand),
    Anchor(RichTextAnchor),
    AnchorLink(RichTextAnchorLink),
    Reference(RichTextReference),
    ReferenceLink(RichTextReferenceLink),
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        RichText::Plain(value)
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        RichText::Plain(value.to_owned())
    }
}

impl From<Vec<RichText>> for RichText {
    fn from(value: Vec<RichText>) -> Self {
        RichText::List(value)
    }
}

impl From<RichTextKind> for RichText {
    fn from(value: RichTextKind) -> Self {
        RichText::Formatted(Box::new(value))
    }
}

/// A bold text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbold).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBold {
    /// The text.
    pub text: RichText,
}

/// An italicized text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextitalic).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextItalic {
    /// The text.
    pub text: RichText,
}

/// An underlined text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextunderline).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextUnderline {
    /// The text.
    pub text: RichText,
}

/// A strikethrough text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextstrikethrough).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextStrikethrough {
    /// The text.
    pub text: RichText,
}

/// A text covered by a spoiler.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextspoiler).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSpoiler {
    /// The text.
    pub text: RichText,
}

/// Formatted date and time.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextdatetime).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextDateTime {
    /// The text.
    pub text: RichText,

    /// The Unix time associated with the entity.
    pub unix_time: i64,

    /// The string that defines the formatting of the date and time.
    pub date_time_format: String,
}

/// A mention of a Telegram user by their identifier.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexttextmention).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextTextMention {
    /// The text.
    pub text: RichText,

    /// The mentioned user.
    // Boxed to keep `RichTextKind` small — `User` is by far the largest field
    // among its variants.
    pub user: Box<User>,
}

/// A subscript text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextsubscript).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSubscript {
    /// The text.
    pub text: RichText,
}

/// A superscript text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextsuperscript).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSuperscript {
    /// The text.
    pub text: RichText,
}

/// A marked text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmarked).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMarked {
    /// The text.
    pub text: RichText,
}

/// A monowidth text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcode).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCode {
    /// The text.
    pub text: RichText,
}

/// A custom emoji.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcustomemoji).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji. Use `getCustomEmojiStickers` to
    /// get full information about the sticker.
    pub custom_emoji_id: CustomEmojiId,

    /// Alternative emoji for the custom emoji.
    pub alternative_text: String,
}

/// A mathematical expression.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmathematicalexpression).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMathematicalExpression {
    /// The expression in LaTeX format.
    pub expression: String,
}

/// A text with a link.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexturl).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextUrl {
    /// The text.
    pub text: RichText,

    /// URL of the link.
    pub url: String,
}

/// A text with an email address.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextemailaddress).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextEmailAddress {
    /// The text.
    pub text: RichText,

    /// The email address.
    pub email_address: String,
}

/// A text with a phone number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextphonenumber).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextPhoneNumber {
    /// The text.
    pub text: RichText,

    /// The phone number.
    pub phone_number: String,
}

/// A text with a bank card number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbankcardnumber).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBankCardNumber {
    /// The text.
    pub text: RichText,

    /// The bank card number.
    pub bank_card_number: String,
}

/// A mention by a username.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmention).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMention {
    /// The text.
    pub text: RichText,

    /// The username.
    pub username: String,
}

/// A hashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexthashtag).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextHashtag {
    /// The text.
    pub text: RichText,

    /// The hashtag.
    pub hashtag: String,
}

/// A cashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcashtag).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCashtag {
    /// The text.
    pub text: RichText,

    /// The cashtag.
    pub cashtag: String,
}

/// A bot command.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbotcommand).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBotCommand {
    /// The text.
    pub text: RichText,

    /// The bot command.
    pub bot_command: String,
}

/// An anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchor).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchor {
    /// The name of the anchor.
    pub name: String,
}

/// A link to an anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchorlink).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchorLink {
    /// The link text.
    pub text: RichText,

    /// The name of the anchor. If empty, the link brings back to the top of
    /// the message.
    pub anchor_name: String,
}

/// A reference.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreference).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextReference {
    /// Text of the reference.
    pub text: RichText,

    /// The name of the reference.
    pub name: String,
}

/// A link to a reference.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreferencelink).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextReferenceLink {
    /// The link text.
    pub text: RichText,

    /// The name of the reference.
    pub reference_name: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn deserialize_plain_string() {
        let rich_text: RichText = serde_json::from_str(r#""hello""#).unwrap();
        assert_eq!(rich_text, RichText::Plain("hello".to_owned()));
    }

    #[test]
    fn deserialize_array_as_list() {
        let rich_text: RichText = serde_json::from_str(r#"["a","b"]"#).unwrap();
        assert_eq!(
            rich_text,
            RichText::List(vec![RichText::Plain("a".to_owned()), RichText::Plain("b".to_owned())])
        );
    }

    #[test]
    fn nested_formatting_round_trip() {
        let original = RichText::Formatted(Box::new(RichTextKind::Bold(RichTextBold {
            text: RichText::Formatted(Box::new(RichTextKind::Italic(RichTextItalic {
                text: RichText::Plain("hello".to_owned()),
            }))),
        })));

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: RichText = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn deserialize_text_less_leaves() {
        let anchor: RichText = serde_json::from_str(r#"{"type":"anchor","name":"x"}"#).unwrap();
        assert_eq!(
            anchor,
            RichText::Formatted(Box::new(RichTextKind::Anchor(RichTextAnchor {
                name: "x".to_owned()
            })))
        );

        let expr: RichText =
            serde_json::from_str(r#"{"type":"mathematical_expression","expression":"x^2"}"#)
                .unwrap();
        assert_eq!(
            expr,
            RichText::Formatted(Box::new(RichTextKind::MathematicalExpression(
                RichTextMathematicalExpression { expression: "x^2".to_owned() }
            )))
        );

        let emoji: RichText = serde_json::from_str(
            r#"{"type":"custom_emoji","custom_emoji_id":"123","alternative_text":"😀"}"#,
        )
        .unwrap();
        assert_eq!(
            emoji,
            RichText::Formatted(Box::new(RichTextKind::CustomEmoji(RichTextCustomEmoji {
                custom_emoji_id: "123".into(),
                alternative_text: "😀".to_owned(),
            })))
        );
    }

    #[test]
    fn unknown_tag_is_rejected() {
        let result: Result<RichText, _> =
            serde_json::from_str(r#"{"type":"nonexistent","text":"x"}"#);
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("data did not match any variant of untagged enum RichText"),
            "unexpected error message: {err}"
        );
    }

    #[test]
    fn size_of_rich_text_is_small() {
        assert!(std::mem::size_of::<RichText>() <= 32);
    }

    // Measured at 64 bytes (the largest variants, e.g. `RichTextDateTime` /
    // `RichTextTextMention`, hold a `RichText` (32 bytes) plus one or two
    // `String`/`Box` fields). Guards against accidentally growing
    // `RichTextKind`'s variants (e.g. un-boxing `RichTextTextMention.user`).
    #[test]
    fn size_of_rich_text_kind_is_bounded() {
        assert!(std::mem::size_of::<RichTextKind>() <= 80);
    }

    #[test]
    fn deserialize_list_nested_in_formatted_text() {
        let rich_text: RichText =
            serde_json::from_str(r#"{"type":"bold","text":["a",{"type":"italic","text":"b"}]}"#)
                .unwrap();

        let expected = RichText::Formatted(Box::new(RichTextKind::Bold(RichTextBold {
            text: RichText::List(vec![
                RichText::Plain("a".to_owned()),
                RichText::Formatted(Box::new(RichTextKind::Italic(RichTextItalic {
                    text: RichText::Plain("b".to_owned()),
                }))),
            ]),
        })));

        assert_eq!(rich_text, expected);
    }

    #[test]
    fn nested_formatting_serializes_to_exact_json() {
        let rich_text = RichText::Formatted(Box::new(RichTextKind::Bold(RichTextBold {
            text: RichText::Formatted(Box::new(RichTextKind::Italic(RichTextItalic {
                text: RichText::Plain("hello".to_owned()),
            }))),
        })));

        let json = serde_json::to_string(&rich_text).unwrap();
        assert_eq!(json, r#"{"type":"bold","text":{"type":"italic","text":"hello"}}"#);
    }

    #[test]
    fn rich_text_is_hashable() {
        let mut set: HashSet<RichText> = HashSet::new();
        set.insert(RichText::Plain("hello".to_owned()));
        assert!(set.contains(&RichText::Plain("hello".to_owned())));
    }

    #[test]
    fn rich_text_kind_variants_serialize() {
        let user = Box::new(User {
            id: crate::types::UserId(42),
            is_bot: false,
            first_name: "First".to_owned(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
            has_topics_enabled: false,
            allows_users_to_create_topics: false,
            can_manage_bots: false,
            supports_guest_queries: false,
            supports_join_request_queries: None,
        });
        let text = || RichText::Plain("x".to_owned());

        let kinds = vec![
            (RichTextKind::Bold(RichTextBold { text: text() }), "bold"),
            (RichTextKind::Italic(RichTextItalic { text: text() }), "italic"),
            (RichTextKind::Underline(RichTextUnderline { text: text() }), "underline"),
            (RichTextKind::Strikethrough(RichTextStrikethrough { text: text() }), "strikethrough"),
            (RichTextKind::Spoiler(RichTextSpoiler { text: text() }), "spoiler"),
            (
                RichTextKind::DateTime(RichTextDateTime {
                    text: text(),
                    unix_time: 0,
                    date_time_format: "".to_owned(),
                }),
                "date_time",
            ),
            (RichTextKind::TextMention(RichTextTextMention { text: text(), user }), "text_mention"),
            (RichTextKind::Subscript(RichTextSubscript { text: text() }), "subscript"),
            (RichTextKind::Superscript(RichTextSuperscript { text: text() }), "superscript"),
            (RichTextKind::Marked(RichTextMarked { text: text() }), "marked"),
            (RichTextKind::Code(RichTextCode { text: text() }), "code"),
            (
                RichTextKind::CustomEmoji(RichTextCustomEmoji {
                    custom_emoji_id: "1".into(),
                    alternative_text: "e".to_owned(),
                }),
                "custom_emoji",
            ),
            (
                RichTextKind::MathematicalExpression(RichTextMathematicalExpression {
                    expression: "x".to_owned(),
                }),
                "mathematical_expression",
            ),
            (RichTextKind::Url(RichTextUrl { text: text(), url: "u".to_owned() }), "url"),
            (
                RichTextKind::EmailAddress(RichTextEmailAddress {
                    text: text(),
                    email_address: "e".to_owned(),
                }),
                "email_address",
            ),
            (
                RichTextKind::PhoneNumber(RichTextPhoneNumber {
                    text: text(),
                    phone_number: "p".to_owned(),
                }),
                "phone_number",
            ),
            (
                RichTextKind::BankCardNumber(RichTextBankCardNumber {
                    text: text(),
                    bank_card_number: "b".to_owned(),
                }),
                "bank_card_number",
            ),
            (
                RichTextKind::Mention(RichTextMention { text: text(), username: "u".to_owned() }),
                "mention",
            ),
            (
                RichTextKind::Hashtag(RichTextHashtag { text: text(), hashtag: "h".to_owned() }),
                "hashtag",
            ),
            (
                RichTextKind::Cashtag(RichTextCashtag { text: text(), cashtag: "c".to_owned() }),
                "cashtag",
            ),
            (
                RichTextKind::BotCommand(RichTextBotCommand {
                    text: text(),
                    bot_command: "b".to_owned(),
                }),
                "bot_command",
            ),
            (RichTextKind::Anchor(RichTextAnchor { name: "n".to_owned() }), "anchor"),
            (
                RichTextKind::AnchorLink(RichTextAnchorLink {
                    text: text(),
                    anchor_name: "n".to_owned(),
                }),
                "anchor_link",
            ),
            (
                RichTextKind::Reference(RichTextReference { text: text(), name: "n".to_owned() }),
                "reference",
            ),
            (
                RichTextKind::ReferenceLink(RichTextReferenceLink {
                    text: text(),
                    reference_name: "n".to_owned(),
                }),
                "reference_link",
            ),
        ];

        assert_eq!(kinds.len(), 25);

        for (kind, wire_type) in kinds {
            assert_eq!(serde_json::to_value(kind).unwrap()["type"], wire_type);
        }
    }
}
