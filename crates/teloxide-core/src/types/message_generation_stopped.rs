use crate::types::{Chat, ThreadId};
use serde::{Deserialize, Serialize};

/// Describes a message generation that was stopped.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct MessageGenerationStopped {
    /// Chat in which the message generation was stopped.
    pub chat: Chat,

    /// Identifier of the message thread in which generation was stopped.
    pub message_thread_id: Option<ThreadId>,

    /// Identifier of the stopped draft.
    pub draft_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_draft_id() {
        let stopped: MessageGenerationStopped = serde_json::from_str(
            r#"{"chat":{"id":1,"type":"private","first_name":"a"},"draft_id":7}"#,
        )
        .unwrap();
        assert_eq!(stopped.draft_id, 7);
        assert_eq!(stopped.message_thread_id, None);
    }
}
