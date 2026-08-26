use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::User;

/// A subscription state reported by Telegram.
///
/// Unknown states are retained so additions to the Bot API do not make update
/// deserialization fail.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename_all = "lowercase"))]
pub enum BotSubscriptionState {
    Canceled,
    Active,
    Failed,
    /// Preserves future Bot API states at runtime. It is skipped from the
    /// test-only schema because Telegram's documented schema is a closed enum.
    #[cfg_attr(test, schemars(skip))]
    Unknown(String),
}

impl BotSubscriptionState {
    fn as_str(&self) -> &str {
        match self {
            Self::Canceled => "canceled",
            Self::Active => "active",
            Self::Failed => "failed",
            Self::Unknown(state) => state,
        }
    }
}

impl Serialize for BotSubscriptionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for BotSubscriptionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "canceled" => Self::Canceled,
            "active" => Self::Active,
            "failed" => Self::Failed,
            state => Self::Unknown(state.to_owned()),
        })
    }
}

/// Describes a change in a bot subscription.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotSubscriptionUpdated {
    /// The subscribed user.
    pub user: User,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Current subscription state.
    pub state: BotSubscriptionState,
}

#[cfg(test)]
mod tests {
    use crate::types::BotSubscriptionState;

    #[test]
    fn preserves_unknown_subscription_state() {
        let state = serde_json::from_str::<BotSubscriptionState>("\"pending\"").unwrap();
        assert_eq!(state, BotSubscriptionState::Unknown("pending".into()));
        assert_eq!(serde_json::to_string(&state).unwrap(), "\"pending\"");
    }

    #[test]
    fn serializes_known_subscription_states() {
        for (state, expected) in [
            (BotSubscriptionState::Canceled, "canceled"),
            (BotSubscriptionState::Active, "active"),
            (BotSubscriptionState::Failed, "failed"),
        ] {
            assert_eq!(serde_json::to_string(&state).unwrap(), format!("\"{expected}\""));
            assert_eq!(
                serde_json::from_str::<BotSubscriptionState>(&format!("\"{expected}\"")).unwrap(),
                state
            );
        }
    }
}
