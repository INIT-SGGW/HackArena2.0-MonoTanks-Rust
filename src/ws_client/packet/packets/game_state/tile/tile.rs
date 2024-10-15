use super::{bullet::Bullet, item::Item, laser::Laser, mine::Mine, tank::Tank};
use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

/// Represents a tile on the map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
pub struct Tile {
    /// Whether the tile is currently visible by you on the map.
    pub visible: bool,
    /// If tile is in a zone, this is the index of the zone it belongs to.
    pub zone_index: Option<u8>,
    /// The specific entities on the tile, determining its content (e.g., empty, wall, tank, bullet).
    pub entities: Vec<TileEntity>,
}

/// Enum representing the possible entities (contents) of a tile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, IsVariant)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum TileEntity {
    /// Represents a tile containing a wall.
    Wall,
    /// A tile containing a tank, where `Tank` represents the associated tank data.
    Tank(Tank),
    /// A tile containing a bullet, where `Bullet` represents the associated bullet data.
    Bullet(Bullet),
    /// A tile containing a laser, where `Laser` represents the associated laser data.
    Laser(Laser),
    /// A tile containing a mine, where `Mine` represents the associated mine data.
    Mine(Mine),
    /// A tile containing an item, where `Item` represents the associated item data.
    Item(Item),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws_client::packet::packets::game_state::tile::{
        bullet::BulletType, direction::Direction, item::ItemType, laser::LaserOrientation,
        tank::Tank, turret::Turret,
    };
    use serde_json;

    #[test]
    fn test_deserialize_wall() {
        let json_data = r#"{"type": "wall"}"#;
        let deserialized: TileEntity = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized, TileEntity::Wall);
    }

    #[test]
    fn test_deserialize_tank() {
        let json_data = r#"{
            "type": "tank",
            "payload": {
                "direction": "right",
                "health": 100,
                "ownerId": "player1",
                "turret": {
                    "bulletCount": 10,
                    "ticksToRegenBullet": 50,
                    "direction": "up"
                },
                "secondaryItem": "radar"
            }
        }"#;
        let deserialized: TileEntity = serde_json::from_str(json_data).unwrap();
        let expected_tank = Tank {
            direction: Direction::Right,
            health: Some(100),
            owner_id: "player1".to_string(),
            turret: Turret {
                bullet_count: Some(10),
                ticks_to_regen_bullet: Some(50),
                direction: Direction::Up,
            },
            secondary_item: Some(ItemType::Radar),
        };
        assert_eq!(deserialized, TileEntity::Tank(expected_tank));
    }

    #[test]
    fn test_deserialize_bullet() {
        let json_data = r#"{
            "type": "bullet",
            "payload": {
                "direction": "down",
                "id": 1,
                "speed": 5.0,
                "type": "basic"
            }
        }"#;
        let deserialized: TileEntity = serde_json::from_str(json_data).unwrap();
        let expected_bullet = Bullet {
            direction: Direction::Down,
            id: 1,
            speed: 5.0,
            bullet_type: BulletType::Basic,
        };
        assert_eq!(deserialized, TileEntity::Bullet(expected_bullet));
    }

    #[test]
    fn test_deserialize_laser() {
        let json_data = r#"{
            "type": "laser",
            "payload": {
                "id": 1,
                "orientation": "horizontal"
            }
        }"#;
        let deserialized: TileEntity = serde_json::from_str(json_data).unwrap();
        let expected_laser = Laser {
            id: 1,
            orientation: LaserOrientation::Horizontal,
        };
        assert_eq!(deserialized, TileEntity::Laser(expected_laser));
    }

    #[test]
    fn test_deserialize_mine() {
        let json_data = r#"{
            "type": "mine",
            "payload": {
                "id": 1,
                "explosionRemainingTicks": 50
            }
        }"#;
        let deserialized: TileEntity = serde_json::from_str(json_data).unwrap();
        let expected_mine = Mine {
            id: 1,
            explosion_remaining_ticks: Some(50),
        };
        assert_eq!(deserialized, TileEntity::Mine(expected_mine));
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let json_data = r#"{"type": "invalid"}"#;
        let deserialized: Result<TileEntity, _> = serde_json::from_str(json_data);
        assert!(deserialized.is_err());
    }

    #[test]
    fn test_deserialize_item() {
        let json_data = r#"{
            "type": "item",
            "payload": {
                "type": "radar"
            }
        }"#;
        let deserialized: TileEntity = serde_json::from_str(json_data).unwrap();
        let expected_item = Item {
            item_type: ItemType::Radar,
        };
        assert_eq!(deserialized, TileEntity::Item(expected_item));
    }
}
