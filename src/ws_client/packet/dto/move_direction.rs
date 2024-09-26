use derive_more::derive::IsVariant;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Represents the direction of movement.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    IntoPrimitive,
    TryFromPrimitive,
    IsVariant,
)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64")]
#[repr(u64)]
pub enum MoveDirection {
    /// Move forward.
    Forward = 0,
    /// Move backward.
    Backward = 1,
}
