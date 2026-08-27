use crate::types::{MessageEntity, OwnedGiftId, UniqueGift};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Describes a service message about a unique gift that was sent or received.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,

    /// Origin of the gift. Currently, either `upgrade` for gifts upgraded from
    /// regular gifts, `transfer` for gifts transferred from other users or
    /// channels, `resale` for gifts bought from other users, `gifted_upgrade`
    /// for upgrades purchased after the gift was sent, or `offer` for gifts
    /// bought or sold through gift purchase offers
    pub origin: UniqueGiftOrigin,

    /// For gifts bought from other users, currency for the price paid for the
    /// gift ("XTR" for Telegram Stars or the ISO 4217 currency code)
    pub last_resale_currency: Option<String>,

    /// For gifts bought from other users, the amount of currency paid for the
    /// gift
    pub last_resale_amount: Option<u32>,

    /// Unique identifier of the received gift for the bot; only present for
    /// gifts received on behalf of business accounts
    pub owned_gift_id: Option<OwnedGiftId>,

    /// Number of Telegram Stars that must be paid to transfer the gift; omitted
    /// if the bot cannot transfer the gift
    pub transfer_star_count: Option<u32>,

    /// Point in time when the gift can be transferred. If it is in the past,
    /// then the gift can be transferred now
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub next_transfer_date: Option<DateTime<Utc>>,

    /// Text of the message that was used to send the gift.
    pub text: Option<String>,

    /// Special entities that appear in the gift text.
    pub entities: Option<Vec<MessageEntity>>,

    /// `true`, if the gift is private.
    pub is_private: Option<bool>,
}

/// Origin of the gift. Currently, either `upgrade` for gifts upgraded from
/// regular gifts, `transfer` for gifts transferred from other users or
/// channels, `resale` for gifts bought from other users, `gifted_upgrade` for
/// upgrades purchased after the gift was sent, or `offer` for gifts bought or
/// sold through gift purchase offers
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum UniqueGiftOrigin {
    Upgrade,
    Transfer,
    Resale,
    GiftedUpgrade,
    Offer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{
            "gift": {
                "gift_id": "gift_id",
                "base_name": "name",
                "name": "name",
                "number": 123,
                "model": {
                    "name": "name",
                    "sticker": {
                        "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                        "file_unique_id": "AgAD2QcAAsEYngs",
                        "file_size": 25734,
                        "width": 463,
                        "height": 512,
                        "type": "regular",
                        "premium_animation": null,
                        "is_animated": false,
                        "is_video": false,
                        "needs_repainting": false
                    },
                    "rarity_per_mille": 123
                },
                "symbol": {
                    "name": "name",
                    "sticker": {
                        "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                        "file_unique_id": "AgAD2QcAAsEYngs",
                        "file_size": 25734,
                        "width": 463,
                        "height": 512,
                        "type": "regular",
                        "premium_animation": null,
                        "is_animated": false,
                        "is_video": false,
                        "needs_repainting": false
                    },
                    "rarity_per_mille": 123
                },
                "backdrop": {
                    "name": "name",
                    "colors": {
                        "center_color": 0,
                        "edge_color": 0,
                        "symbol_color": 0,
                        "text_color": 0
                    },
                    "rarity_per_mille": 123
                }
            },
            "origin": "upgrade"
        }"#;

        let unique_gift_info: UniqueGiftInfo = serde_json::from_str(data).unwrap();
        assert_eq!(unique_gift_info.origin, UniqueGiftOrigin::Upgrade);
        assert_eq!(unique_gift_info.gift.name, "name");
        assert_eq!(unique_gift_info.gift.backdrop.rarity_per_mille, 123);
    }

    #[test]
    fn deserializes_text_entities_and_privacy() {
        let data = r#"{
            "gift": {
                "gift_id":"gift","base_name":"name","name":"name","number":1,
                "model":{"name":"model","sticker":{"file_id":"id","file_unique_id":"unique","width":1,"height":1,"type":"regular","is_animated":false,"is_video":false,"needs_repainting":false},"rarity_per_mille":1},
                "symbol":{"name":"symbol","sticker":{"file_id":"id","file_unique_id":"unique","width":1,"height":1,"type":"regular","is_animated":false,"is_video":false,"needs_repainting":false},"rarity_per_mille":1},
                "backdrop":{"name":"backdrop","colors":{"center_color":0,"edge_color":0,"symbol_color":0,"text_color":0},"rarity_per_mille":1}
            },
            "origin":"upgrade","text":"A gift","entities":[],"is_private":true
        }"#;
        let info: UniqueGiftInfo = serde_json::from_str(data).unwrap();

        assert_eq!(info.text.as_deref(), Some("A gift"));
        assert_eq!(info.entities, Some(vec![]));
        assert_eq!(info.is_private, Some(true));
        assert_eq!(serde_json::to_value(info).unwrap()["is_private"], true);
    }
}
