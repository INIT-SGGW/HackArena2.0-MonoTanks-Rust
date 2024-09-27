use super::direction::Direction;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Represents a turret with specific attributes related to its ammunition and orientation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Turret {
    /// The current count of bullets available for the turret. It is only Some for your own turret.
    pub bullet_count: Option<i64>,

    /// The progress of bullet regeneration, represented as a percentage. It is only Some for your own turret.
    pub bullet_regen_progress: Option<f64>,

    /// The direction the turret is facing.
    pub direction: Direction,
}
