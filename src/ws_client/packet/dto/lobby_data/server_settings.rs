use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct ServerSettings {
    pub grid_dimension: u32,
    pub number_of_players: u32,
    pub seed: u32,
    pub broadcast_interval: u32,
    pub eager_broadcast: bool,
}
