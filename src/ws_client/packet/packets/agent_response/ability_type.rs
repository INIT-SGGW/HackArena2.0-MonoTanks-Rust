use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64", try_from = "u64")]
#[repr(u64)]
pub enum AbilityType {
    FireBullet = 0,
    UseLaser = 1,
    FireDoubleBullet = 2,
    UseRadar = 3,
    DropMine = 4,
}
