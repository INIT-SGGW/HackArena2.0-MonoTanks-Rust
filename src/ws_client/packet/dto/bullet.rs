use super::direction::Direction;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Bullet {
    pub direction: Direction,
    pub id: i64,
    pub speed: f64,
}
