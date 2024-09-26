pub mod server_settings;

use super::player::Player;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use server_settings::ServerSettings;

/// Represents the data for a game lobby.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LobbyData {
    /// A unique identifier for the player.
    pub player_id: String,

    /// A list of players currently in the lobby.
    pub players: Vec<Player>,

    /// The settings for the server hosting the lobby.
    pub server_settings: ServerSettings,
}
