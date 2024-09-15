use super::{bullet::Bullet, tank::Tank};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum TilePayload {
    Empty,
    Wall,
    Tank(Tank),
    Bullet(Bullet),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Tile {
    pub visible: bool,
    pub zone_index: Option<u8>,
    pub payload: TilePayload,
}
