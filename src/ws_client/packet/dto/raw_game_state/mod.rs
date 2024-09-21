pub mod map;

use derive_more::derive::Constructor;
use map::Map;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct RawGameState {
    pub tick: u64,
    pub player_id: String,
    pub players: Vec<Player>,
    pub map: Map,
}
