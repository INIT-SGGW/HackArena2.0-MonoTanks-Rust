use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub nickname: String,
    pub color: u64,
    pub ping: Option<u64>,
    pub score: Option<u64>,
    pub ticks_to_regen: Option<u64>,
}
