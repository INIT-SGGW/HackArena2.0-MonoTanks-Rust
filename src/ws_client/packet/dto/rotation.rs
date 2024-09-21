use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64")]
#[repr(u64)]
pub enum Rotation {
    Left = 0,
    Right = 1,
}
