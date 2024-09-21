pub mod server_settings;

use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};
use server_settings::ServerSettings;

use super::player::Player;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LobbyData {
    pub player_id: String,
    pub players: Vec<Player>,
    pub server_settings: ServerSettings,
}

// TODO: Can we use a macro to generate this?
impl TryFrom<serde_json::Value> for LobbyData {
    type Error = String;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| e.to_string())
    }
}
