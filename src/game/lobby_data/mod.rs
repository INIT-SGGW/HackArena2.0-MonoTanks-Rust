pub mod server_settings;

use super::game_state::player::Player;
use serde::Deserialize;
use server_settings::ServerSettings;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyData {
    pub player_id: String,
    pub players: Vec<Player>,
    pub server_settings: ServerSettings,
}

impl TryFrom<serde_json::Value> for LobbyData {
    type Error = String;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| e.to_string())
    }
}
