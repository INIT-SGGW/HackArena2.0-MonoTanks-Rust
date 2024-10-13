use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive, IsVariant)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64", try_from = "u64")]
#[repr(u64)]
pub enum ItemType {
    Unknown = 0,
    Laser = 1,
    DoubleBullet = 2,
    Radar = 3,
    Mine = 4,
}
