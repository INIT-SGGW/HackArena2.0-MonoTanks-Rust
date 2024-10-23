use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AbilityType {
    FireBullet,
    UseLaser,
    FireDoubleBullet,
    UseRadar,
    DropMine,
}
