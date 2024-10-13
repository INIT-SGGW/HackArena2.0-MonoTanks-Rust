use derive_more::derive::{Constructor, IsVariant};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Laser {
    pub id: i64,
    pub orientation: LaserOrientation,
}

#[derive(
    Debug,
    IntoPrimitive,
    TryFromPrimitive,
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
#[serde(into = "u64", try_from = "u64")]
#[repr(u64)]
pub enum LaserOrientation {
    Horizontal,
    Vertical,
}
