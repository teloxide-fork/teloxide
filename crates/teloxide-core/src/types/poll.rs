use crate::types::{Chat, MessageEntity, PollMedia, PollType, Seconds, User};

use chrono::{DateTime, Utc};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

/// Unique poll identifier.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct PollId(pub String);

/// This object contains information about a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#poll).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Poll {
    /// Unique poll identifier.
    pub id: PollId,

    /// Poll question, 1-300 characters.
    pub question: String,

    /// Special entities that appear in the question. Currently, only custom
    /// emoji entities are allowed in poll questions
    pub question_entities: Option<Vec<MessageEntity>>,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// `true`, if the poll is closed.
    pub is_closed: bool,

    /// Total number of users that voted in the poll
    pub total_voter_count: u32,

    /// True, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub poll_type: PollType,

    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// `true`, if the poll allows to change the chosen answer options.
    pub allows_revoting: bool,

    /// 0-based identifiers of the correct answer options. Available only for
    /// polls in the quiz mode, which are closed, or was sent (not
    /// forwarded) by the bot or to the private chat with the bot.
    pub correct_option_ids: Option<Vec<u8>>,

    /// Text that is shown when a user chooses an incorrect answer or taps on
    /// the lamp icon in a quiz-style poll, 0-200 characters.
    pub explanation: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc. that appear in
    /// the explanation.
    pub explanation_entities: Option<Vec<MessageEntity>>,

    /// Amount of time in seconds the poll will be active after creation.
    pub open_period: Option<Seconds>,

    /// Point in time when the poll will be automatically closed.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub close_date: Option<DateTime<Utc>>,

    /// Description of the poll; for polls inside the Message object only.
    pub description: Option<String>,

    /// Special entities that appear in the description.
    pub description_entities: Option<Vec<MessageEntity>>,

    /// `true`, if the poll is available only to chat members.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub members_only: bool,

    /// ISO 3166-1 alpha-2 country codes of countries where the poll is
    /// available.
    pub country_codes: Option<Vec<String>>,

    /// Media accompanying the poll explanation.
    pub explanation_media: Option<PollMedia>,

    /// Media accompanying the poll.
    pub media: Option<PollMedia>,
}

/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOption {
    /// Persistent identifier of the option.
    pub persistent_id: String,

    /// Option text, 1-100 characters.
    pub text: String,

    /// Special entities that appear in the option text. Currently, only custom
    /// emoji entities are allowed in poll option texts
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Number of users that voted for this option.
    pub voter_count: u32,

    /// User who added the option.
    pub added_by_user: Option<User>,

    /// Chat that added the option.
    pub added_by_chat: Option<Chat>,

    /// Point in time when the option was added.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub addition_date: Option<DateTime<Utc>>,

    /// Media accompanying the poll option.
    pub media: Option<PollMedia>,
}

impl Poll {
    /// Returns all users that are "contained" in this `Poll`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use crate::util::{flatten, mentioned_users_from_entities};

        flatten(self.explanation_entities.as_deref().map(mentioned_users_from_entities))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "allows_multiple_answers": false,
            "allows_revoting": false,
            "id": "5377643193141559299",
            "is_anonymous": true,
            "is_closed": false,
            "options": [
                {
                    "persistent_id": "1",
                    "text": "1",
                    "voter_count": 1
                },
                {
                    "persistent_id": "2",
                    "text": "2",
                    "voter_count": 0
                },
                {
                    "persistent_id": "3",
                    "text": "3",
                    "voter_count": 0
                },
                {
                    "persistent_id": "4",
                    "text": "4",
                    "voter_count": 0
                },
                {
                    "persistent_id": "5",
                    "text": "5",
                    "voter_count": 0
                }
            ],
            "question": "Rate me from 1 to 5.",
            "total_voter_count": 1,
            "type": "regular"
        }
        "#;
        serde_json::from_str::<Poll>(data).unwrap();
    }

    #[test]
    fn deserialize_media_fields() {
        let poll: Poll = serde_json::from_str(
            r#"{
                "id":"poll",
                "question":"Question",
                "options":[{"persistent_id":"option","text":"Option","voter_count":0,"media":{}}],
                "is_closed":false,
                "total_voter_count":0,
                "is_anonymous":true,
                "type":"regular",
                "allows_multiple_answers":false,
                "allows_revoting":false,
                "members_only":true,
                "country_codes":["US"],
                "explanation_media":{},
                "media":{}
            }"#,
        )
        .unwrap();

        assert!(poll.members_only);
        assert_eq!(poll.country_codes, Some(vec!["US".to_owned()]));
        assert!(poll.explanation_media.is_some());
        assert!(poll.media.is_some());
        assert!(poll.options[0].media.is_some());
    }
}
