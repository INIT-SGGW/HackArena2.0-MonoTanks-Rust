use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Laser {
    pub id: i64,
    pub orientation: LaserOrientation,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IsVariant,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum LaserOrientation {
    Horizontal,
    Vertical,
}
