use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Represents the direction of rotation.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive,
)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64")]
#[repr(u64)]
pub enum Rotation {
    /// Rotate to the left.
    Left = 0,
    /// Rotate to the right.
    Right = 1,
}
