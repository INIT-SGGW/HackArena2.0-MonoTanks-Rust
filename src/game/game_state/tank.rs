use super::{direction::Direction, turret::Turret};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tank {
    pub direction: Direction,
    pub health: Option<i64>,
    pub owner_id: String,
    pub turret: Turret,
}
