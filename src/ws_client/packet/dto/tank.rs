use super::{direction::Direction, turret::Turret};
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Tank {
    pub direction: Direction,
    pub health: Option<i64>,
    pub owner_id: String,
    pub turret: Turret,
}
