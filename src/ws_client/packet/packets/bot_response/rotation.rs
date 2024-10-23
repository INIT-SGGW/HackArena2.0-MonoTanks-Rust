use serde::{Deserialize, Serialize};

/// Represents the direction of rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Rotation {
    /// Rotate to the left.
    Left,
    /// Rotate to the right.
    Right,
}
