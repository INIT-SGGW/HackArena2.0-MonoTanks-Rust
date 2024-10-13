use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, IsVariant)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Unknown,
    Laser,
    DoubleBullet,
    Radar,
    Mine,
}
