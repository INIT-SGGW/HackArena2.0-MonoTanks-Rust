use super::direction::Direction;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Represents a turret with specific attributes related to its ammunition and orientation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Turret {
    /// The current count of bullets available for the turret. It is only Some for your own turret.
    pub bullet_count: Option<i64>,

    /// The number of ticks remaining until the turret regenerates a bullet. It is only Some for your own turret.
    pub ticks_to_regen_bullet: Option<i64>,

    /// The direction the turret is facing.
    pub direction: Direction,
}
