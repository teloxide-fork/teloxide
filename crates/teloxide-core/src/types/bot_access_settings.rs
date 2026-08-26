use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object describes the access settings of a managed bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    /// `true`, if the bot's access is restricted.
    pub is_access_restricted: bool,

    /// Information about users that were added by the bot.
    pub added_users: Option<Vec<User>>,
}

#[cfg(test)]
mod tests {
    use crate::types::BotAccessSettings;

    #[test]
    fn deserialize() {
        let settings: BotAccessSettings =
            serde_json::from_str(r#"{"is_access_restricted":true,"added_users":[]}"#).unwrap();

        assert!(settings.is_access_restricted);
        assert_eq!(settings.added_users, Some(vec![]));
    }
}
