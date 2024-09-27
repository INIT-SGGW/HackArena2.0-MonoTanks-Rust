use super::{bullet::Bullet, tank::Tank};
use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

/// Represents a tile on the map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
pub struct Tile {
    /// Whether the tile is currently visible by you on the map.
    pub visible: bool,
    /// If tile is in a zone, this is the index of the zone it belongs to.
    pub zone_index: Option<u8>,
    /// The specific payload of the tile, determining its content (e.g., empty, wall, tank, bullet).
    pub payload: TilePayload,
}

/// Enum representing the possible contents (payloads) of a tile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, IsVariant)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum TilePayload {
    /// Represents an empty tile with no contents.
    Empty,
    /// Represents a tile containing a wall.
    Wall,
    /// A tile containing a tank, where `Tank` represents the associated tank data.
    Tank(Tank),
    /// A tile containing a bullet, where `Bullet` represents the associated bullet data.
    Bullet(Bullet),
}
