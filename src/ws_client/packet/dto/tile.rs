use super::{bullet::Bullet, tank::Tank};
use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
pub struct Tile {
    pub visible: bool,
    pub zone_index: Option<u8>,
    pub payload: TilePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, IsVariant)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum TilePayload {
    Empty,
    Wall,
    Tank(Tank),
    Bullet(Bullet),
}
