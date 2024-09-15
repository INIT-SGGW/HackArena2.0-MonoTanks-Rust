use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, IntoPrimitive, TryFromPrimitive)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64")]
#[repr(u64)]
pub enum MoveDirection {
    Forward = 0,
    Backward = 1,
}
