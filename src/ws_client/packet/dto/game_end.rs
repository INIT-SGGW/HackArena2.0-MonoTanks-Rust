use super::player::Player;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameEnd {
    pub players: Vec<Player>,
}
