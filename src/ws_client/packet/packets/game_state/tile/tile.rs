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

#[cfg(test)]
mod tests {
    use crate::ws_client::packet::packets::game_state::tile::{direction::Direction, turret::Turret};

    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_wall() {
        let json_data = r#"{"type": "wall"}"#;
        let deserialized: TilePayload = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized, TilePayload::Wall);
    }

    #[test]
    fn test_deserialize_tank() {
        let json_data = r#"{
            "type": "tank",
            "payload": {
                "direction": 1,
                "health": 100,
                "ownerId": "player1",
                "turret": {
                    "bulletCount": 10,
                    "ticksToRegenBullet": 50,
                    "direction": 0
                }
            }
        }"#;
        let deserialized: TilePayload = serde_json::from_str(json_data).unwrap();
        let expected_tank = Tank {
            direction: Direction::Right,
            health: Some(100),
            owner_id: "player1".to_string(),
            turret: Turret {
                bullet_count: Some(10),
                ticks_to_regen_bullet: Some(50),
                direction: Direction::Up,
            },
        };
        assert_eq!(deserialized, TilePayload::Tank(expected_tank));
    }

    #[test]
    fn test_deserialize_bullet() {
        let json_data = r#"{
            "type": "bullet",
            "payload": {
                "direction": 2,
                "id": 1,
                "speed": 5.0
            }
        }"#;
        let deserialized: TilePayload = serde_json::from_str(json_data).unwrap();
        let expected_bullet = Bullet {
            direction: Direction::Down,
            id: 1,
            speed: 5.0,
        };
        assert_eq!(deserialized, TilePayload::Bullet(expected_bullet));
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let json_data = r#"{"type": "invalid"}"#;
        let deserialized: Result<TilePayload, _> = serde_json::from_str(json_data);
        assert!(deserialized.is_err());
    }
}