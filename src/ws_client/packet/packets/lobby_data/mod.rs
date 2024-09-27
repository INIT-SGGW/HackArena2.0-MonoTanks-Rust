pub mod lobby_player;
pub mod server_settings;

use derive_more::Constructor;
use lobby_player::LobbyPlayer;
use serde::{Deserialize, Serialize};
use server_settings::ServerSettings;

/// Represents the data for a game lobby.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LobbyData {
    /// A unique identifier for the player.
    pub player_id: String,

    /// A list of players currently in the lobby.
    pub players: Vec<LobbyPlayer>,

    /// The settings for the server hosting the lobby.
    pub server_settings: ServerSettings,
}
