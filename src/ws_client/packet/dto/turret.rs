use super::direction::Direction;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Turret {
    pub bullet_count: Option<i64>,
    pub bullet_regen_progress: Option<f64>,
    pub direction: Direction,
}
