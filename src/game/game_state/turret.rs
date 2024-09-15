use super::direction::Direction;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Turret {
    pub bullet_count: Option<i64>,
    pub bullet_regen_progress: Option<f64>,
    pub direction: Direction,
}

impl Turret {
    pub fn is_ready(&self) -> bool {
        self.bullet_count.is_some_and(|v| v > 0)
    }

    pub fn is_full(&self) -> bool {
        self.bullet_count.is_some_and(|v| v == 3)
    }
}
