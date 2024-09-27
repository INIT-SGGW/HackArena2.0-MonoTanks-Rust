use super::direction::Direction;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Represents a bullet in the game.
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Bullet {
    /// The direction in which the bullet is fired.
    pub direction: Direction,

    /// A unique identifier for the bullet.
    pub id: i64,

    /// The speed of the bullet, indicating how fast it moves. Its unit is tiles per second.
    pub speed: f64,
}
