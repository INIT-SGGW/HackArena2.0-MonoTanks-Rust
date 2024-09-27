pub mod map;

use derive_more::derive::Constructor;
use map::RawMap;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct RawGameState {
    pub id: String,
    pub tick: u64,
    pub players: Vec<Player>,
    pub map: RawMap,
}
