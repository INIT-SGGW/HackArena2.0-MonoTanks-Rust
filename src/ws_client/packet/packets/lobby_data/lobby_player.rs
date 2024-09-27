use derive_more::Constructor;
use serde::{Deserialize, Serialize};

/// Represents a player in game lobby.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct LobbyPlayer {
    /// A unique identifier for the player.
    pub id: String,

    /// The player's chosen nickname or alias.
    pub nickname: String,

    /// Represents the player's color, used in visual representation as a color for nickname and tank.
    pub color: u64,
}
