use derive_more::Constructor;
use serde::{Deserialize, Serialize};

/// Represents a player in the game.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// A unique identifier for the player.
    pub id: String,

    /// The player's chosen nickname or alias.
    pub nickname: String,

    /// Represents the player's color, used in visual representation as a color for nickname and tank.
    pub color: u64,

    /// The player's current ping, representing latency, if available.
    pub ping: Option<u64>,

    /// The player's score in the game, if available.
    pub score: Option<u64>,

    /// Number of ticks (time units) remaining until the player's health or resource regenerates, if applicable. This is when player is dead.
    pub ticks_to_regen: Option<u64>,
}
