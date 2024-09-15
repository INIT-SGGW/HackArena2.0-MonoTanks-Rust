use serde::Deserialize;

use super::direction::Direction;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bullet {
    pub direction: Direction,
    pub id: i64,
    pub speed: f64,
}
