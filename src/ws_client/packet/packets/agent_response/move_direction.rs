use derive_more::derive::IsVariant;
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
    IsVariant,
)]
#[serde(rename_all = "camelCase")]
pub enum MoveDirection {
    /// Move forward.
    Forward,
    /// Move backward.
    Backward,
}
