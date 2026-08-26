use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatId, ChatInviteLink, User};

/// Represents a join request sent to a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request.
    /// The bot can use this identifier for 5 minutes to send messages until
    /// the join request is processed, assuming no other administrator
    /// contacted the user.
    pub user_chat_id: ChatId,
    /// Date the request was sent in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub date: DateTime<Utc>,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
    /// Identifier of the join request query. If present, the bot must call
    /// [`SendChatJoinRequestWebApp`] or directly call
    /// [`AnswerChatJoinRequestQuery`] within 10 seconds.
    ///
    /// [`SendChatJoinRequestWebApp`]: crate::payloads::SendChatJoinRequestWebApp
    /// [`AnswerChatJoinRequestQuery`]: crate::payloads::AnswerChatJoinRequestQuery
    pub query_id: Option<String>,
}

/// Result of a chat join request query, used with
/// [`AnswerChatJoinRequestQuery`].
///
/// [`AnswerChatJoinRequestQuery`]: crate::payloads::AnswerChatJoinRequestQuery
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ChatJoinRequestQueryResult {
    /// Approve the chat join request.
    Approve,
    /// Decline the chat join request.
    Decline,
    /// Leave the decision to other administrators.
    Queue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_id_deserialize() {
        let json = r#"{
            "chat": {"id": -1, "type": "supergroup", "title": "Test"},
            "from": {"id": 1, "is_bot": false, "first_name": "User"},
            "user_chat_id": 1,
            "date": 1720708004,
            "query_id": "some-query-id"
        }"#;
        let request: ChatJoinRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.query_id.as_deref(), Some("some-query-id"));
    }

    #[test]
    fn query_id_absent() {
        let json = r#"{
            "chat": {"id": -1, "type": "supergroup", "title": "Test"},
            "from": {"id": 1, "is_bot": false, "first_name": "User"},
            "user_chat_id": 1,
            "date": 1720708004
        }"#;
        let request: ChatJoinRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.query_id, None);
    }

    #[test]
    fn answer_chat_join_request_query_serialize() {
        use crate::payloads::AnswerChatJoinRequestQuery;

        let payload =
            AnswerChatJoinRequestQuery::new("some-query-id", ChatJoinRequestQueryResult::Approve);
        let value = serde_json::to_value(&payload).unwrap();
        assert_eq!(value["chat_join_request_query_id"], "some-query-id");
        assert_eq!(value["result"], "approve");

        let payload =
            AnswerChatJoinRequestQuery::new("some-query-id", ChatJoinRequestQueryResult::Decline);
        assert_eq!(serde_json::to_value(&payload).unwrap()["result"], "decline");

        let payload =
            AnswerChatJoinRequestQuery::new("some-query-id", ChatJoinRequestQueryResult::Queue);
        assert_eq!(serde_json::to_value(&payload).unwrap()["result"], "queue");
    }

    #[test]
    fn send_chat_join_request_web_app_serialize() {
        use crate::payloads::SendChatJoinRequestWebApp;

        let payload = SendChatJoinRequestWebApp::new("some-query-id", "https://example.com/app");
        let value = serde_json::to_value(&payload).unwrap();
        assert_eq!(value["chat_join_request_query_id"], "some-query-id");
        assert_eq!(value["web_app_url"], "https://example.com/app");
    }

    #[test]
    fn chat_join_request_query_result_serialize() {
        assert_eq!(
            serde_json::to_string(&ChatJoinRequestQueryResult::Approve).unwrap(),
            r#""approve""#
        );
        assert_eq!(
            serde_json::to_string(&ChatJoinRequestQueryResult::Decline).unwrap(),
            r#""decline""#
        );
        assert_eq!(
            serde_json::to_string(&ChatJoinRequestQueryResult::Queue).unwrap(),
            r#""queue""#
        );
    }
}
