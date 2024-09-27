use super::game_end_player::GameEndPlayer;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Represents the end state of a game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameEnd {
    /// The list of players at the end of the game.
    pub players: Vec<GameEndPlayer>,
}
