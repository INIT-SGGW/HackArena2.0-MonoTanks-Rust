use super::{
    player::Player,
    tile::{Tile, TilePayload},
    zone::Zone,
    GameState,
};
use serde_json::Value;

impl TryFrom<serde_json::Value> for GameState {
    type Error = String;

    fn try_from(payload: serde_json::Value) -> Result<Self, Self::Error> {
        let time = parse_time(&payload)?;
        let players = parse_players(&payload)?;

        let zones = parse_zones(&payload)?;
        let map = parse_map(&payload, &zones)?;

        Ok(GameState::new(payload, time, players, map, zones))
    }
}

fn parse_time(payload: &serde_json::Value) -> Result<f64, String> {
    payload
        .get("time")
        .ok_or("Missing time field")?
        .as_f64()
        .ok_or("time is not a float".to_owned())
}

fn parse_players(payload: &Value) -> Result<Vec<Player>, String> {
    payload
        .get("players")
        .ok_or("Missing players".into())
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| format!("Failed to parse players: {}", e))
        })
}

fn parse_zones(payload: &serde_json::Value) -> Result<Vec<Zone>, String> {
    payload
        .get("map")
        .and_then(|grid| grid.get("zones"))
        .ok_or("Missing or invalid zones field".to_string())
        .and_then(|zones| {
            serde_json::from_value::<Vec<Zone>>(zones.clone())
                .map_err(|e| format!("Failed to parse zones: {}", e))
        })
}

fn parse_map(payload: &serde_json::Value, zones: &[Zone]) -> Result<Vec<Vec<Tile>>, String> {
    let tiles = payload
        .get("map")
        .ok_or("Missing map field")?
        .get("tiles")
        .ok_or("Missing tiles field")?
        .as_array()
        .ok_or("tiles is not an array")?;

    let num_of_columns = tiles.len();
    let num_of_rows = tiles[0].as_array().ok_or("Map row is not an array")?.len();

    let mut map = vec![
        vec![
            Tile {
                payload: TilePayload::Empty,
                visible: false,
                zone_index: None,
            };
            num_of_rows
        ];
        num_of_columns
    ];

    for (x, column) in tiles.iter().enumerate() {
        for (y, tile) in column
            .as_array()
            .ok_or("Map row is not an array")?
            .iter()
            .enumerate()
        {
            map[y][x].payload = parse_map_tile(tile)
                .map_err(|e| format!("Failed to parse tile at ({}, {}): {}", x, y, e))?;
        }
    }

    let visibility_grid = payload
        .get("map")
        .ok_or("Missing map field")?
        .get("visibility")
        .ok_or("Missing visibility field")?
        .as_array()
        .ok_or("visibility is not an array")?;

    for (y, row) in visibility_grid.iter().enumerate() {
        let row = row.as_str().ok_or("Visibility row is not a string")?;
        for (x, visible) in row.chars().enumerate() {
            map[y][x].visible = visible == '1';
        }
    }

    for zone in zones {
        for x in zone.x..zone.x + zone.width {
            for y in zone.y..zone.y + zone.height {
                map[y as usize][x as usize].zone_index = Some(zone.index);
            }
        }
    }

    Ok(map)
}

fn parse_map_tile(tile: &Value) -> Result<TilePayload, String> {
    let items = tile.as_array().ok_or("Map column is not an array")?;

    let result = if items.is_empty() {
        TilePayload::Empty
    } else {
        let item = items.first().ok_or("No items in the tile array")?;
        serde_json::from_value(item.clone()).map_err(|e| format!("Failed to parse tile: {}", e))?
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::game::game_state::{
        direction::Direction, tank::Tank, turret::Turret, zone::ZoneStatus,
    };

    use super::*;

    #[test]
    fn test_parse_time() {
        let payload = serde_json::json!({
            "time": 123.456
        });

        let time = parse_time(&payload).unwrap();
        assert_eq!(time, 123.456);
    }

    #[test]
    fn test_parse_players() {
        let payload = serde_json::json!({
            "players": [
                {
                    "id": "e15a3449-bb72-4a3a-a5fc-42292189311e",
                    "nickname": "Player 1",
                    "color": 4294944256u64,
                    "ping": 0
                },
                {
                    "id": "953d0f9d-7126-4eb4-b48e-cfc6862033df",
                    "nickname": "Player 2",
                    "color": 4294925049u64,
                    "ping": 0
                },
                {
                    "id": "fd76a5bd-0c05-4b05-8f61-9f68a7bc2b1a",
                    "nickname": "Player 3",
                    "color": 4279933945u64,
                    "ping": 0,
                    "score": 0
                }
            ]
        });

        let players = parse_players(&payload).unwrap();
        assert_eq!(players.len(), 3);
        assert_eq!(
            players[0],
            Player {
                id: "e15a3449-bb72-4a3a-a5fc-42292189311e".to_owned(),
                nickname: "Player 1".to_owned(),
                color: 4294944256u64,
                ping: 0,
                score: None,
            }
        );
        assert_eq!(
            players[1],
            Player {
                id: "953d0f9d-7126-4eb4-b48e-cfc6862033df".to_owned(),
                nickname: "Player 2".to_owned(),
                color: 4294925049u64,
                ping: 0,
                score: None,
            }
        );
        assert_eq!(
            players[2],
            Player {
                id: "fd76a5bd-0c05-4b05-8f61-9f68a7bc2b1a".to_owned(),
                nickname: "Player 3".to_owned(),
                color: 4279933945u64,
                ping: 0,
                score: Some(0),
            }
        );
    }

    // Test parse_zones
    #[test]
    fn test_parse_zones() {
        let payload = serde_json::json!({
            "map": {
                "zones": [
                    {
                        "x": 5,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "type": "neutral"
                        }
                    },
                    {
                        "x": 5,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "remainingTicks": 93,
                            "playerId": "508b7e1d-68c4-48c4-8a77-495dc9fd74ff",
                            "type": "beingCaptured"
                        }
                    },
                    {
                        "x": 5,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "playerId": "508b7e1d-68c4-48c4-8a77-495dc9fd74ff",
                            "type": "captured"
                        }
                    },
                    {
                        "x": 5,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "capturedById": null,
                            "type": "beingContested"
                        }
                    },
                    {
                       "x": 5,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "capturedById": "077e53b0-2eac-47c6-a5a1-d6ce04fc0ba8",
                            "type": "beingContested"
                        }
                    },
                    {
                        "x": 5,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "remainingTicks": 59,
                            "capturedById": "077e53b0-2eac-47c6-a5a1-d6ce04fc0ba8",
                            "retakenById": "2f5aa85d-9097-4cbe-acba-437fb900c9b5",
                            "type": "beingRetaken"
                        }
                    }
                ]
            }
        });

        let zones = parse_zones(&payload).unwrap();
        assert_eq!(zones.len(), 6);
        assert_eq!(
            zones[0],
            Zone {
                x: 5,
                y: 1,
                width: 4,
                height: 4,
                index: 65,
                status: ZoneStatus::Neutral,
            }
        );
        assert_eq!(
            zones[1],
            Zone {
                x: 5,
                y: 1,
                width: 4,
                height: 4,
                index: 65,
                status: ZoneStatus::BeingCaptured {
                    remaining_ticks: 93,
                    player_id: "508b7e1d-68c4-48c4-8a77-495dc9fd74ff".to_owned(),
                },
            }
        );
        assert_eq!(
            zones[2],
            Zone {
                x: 5,
                y: 1,
                width: 4,
                height: 4,
                index: 65,
                status: ZoneStatus::Captured {
                    player_id: "508b7e1d-68c4-48c4-8a77-495dc9fd74ff".to_owned(),
                },
            }
        );
        assert_eq!(
            zones[3],
            Zone {
                x: 5,
                y: 1,
                width: 4,
                height: 4,
                index: 65,
                status: ZoneStatus::BeingContested {
                    captured_by_id: None,
                },
            }
        );
        assert_eq!(
            zones[4],
            Zone {
                x: 5,
                y: 1,
                width: 4,
                height: 4,
                index: 65,
                status: ZoneStatus::BeingContested {
                    captured_by_id: Some("077e53b0-2eac-47c6-a5a1-d6ce04fc0ba8".to_owned()),
                },
            }
        );
        assert_eq!(
            zones[5],
            Zone {
                x: 5,
                y: 1,
                width: 4,
                height: 4,
                index: 65,
                status: ZoneStatus::BeingRetaken {
                    remaining_ticks: 59,
                    captured_by_id: "077e53b0-2eac-47c6-a5a1-d6ce04fc0ba8".to_owned(),
                    retaken_by_id: "2f5aa85d-9097-4cbe-acba-437fb900c9b5".to_owned(),
                },
            }
        );
    }

    // Test parse_map
    #[test]
    fn test_parse_map() {
        let payload = serde_json::json!({
            "map": {
                "tiles": [
                    [
                        [],
                        [],
                        [],
                        [],
                        [],
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        [
                            {
                                "type": "wall"
                            }
                        ]
                    ],
                    [
                        [
                            {
                                "type": "tank",
                                "payload": {
                                    "ownerId": "bce2b37c-7c3d-4a59-bba6-0458caa24b04",
                                    "direction": 1,
                                    "turret": {
                                        "direction": 1,
                                        "bulletCount": 3,
                                        "bulletRegenProgress": null
                                    },
                                    "health": 100
                                }
                            }
                        ],
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        [],
                        [],
                        [],
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        [
                            {
                                "type": "wall"
                            }
                        ]
                    ],
                    [
                        [],
                        [],
                        [],
                        [],
                        [],
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        []
                    ],
                    [
                        [],
                        [],
                        [],
                        [],
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        [],
                        []
                    ],
                    [
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        [],
                        [],
                        [],
                        [],
                        [],
                        []
                    ],
                    [
                        [],
                        [],
                        [
                            {
                                "type": "wall"
                            }
                        ],
                        [],
                        [],
                        [],
                        []
                    ],
                    [
                        [],
                        [],
                        [],
                        [],
                        [],
                        [],
                        []
                    ]
                ],
                "zones": [
                    {
                        "x": 1,
                        "y": 1,
                        "width": 4,
                        "height": 4,
                        "index": 65,
                        "status": {
                            "type": "neutral"
                        }
                    }
                ],
                "visibility": [
                    "0111000",
                    "0011111",
                    "0001100",
                    "0000110",
                    "0000011",
                    "0000001",
                    "0000001"
                ]
            }
        });

        let zones = parse_zones(&payload).unwrap();
        let map = parse_map(&payload, &zones).unwrap();

        assert_eq!(map.len(), 7);
        assert_eq!(map[0].len(), 7);
        assert_eq!(map[0][0].payload, TilePayload::Empty);
        assert_eq!(map[5][0].payload, TilePayload::Wall);
        assert_eq!(map[6][0].payload, TilePayload::Wall);
        assert_eq!(map[1][1].payload, TilePayload::Wall);
        assert_eq!(
            map[0][1].payload,
            TilePayload::Tank(Tank {
                owner_id: "bce2b37c-7c3d-4a59-bba6-0458caa24b04".to_owned(),
                direction: Direction::Right,
                turret: Turret {
                    direction: Direction::Right,
                    bullet_count: Some(3),
                    bullet_regen_progress: None,
                },
                health: Some(100),
            })
        );

        assert_eq!(map[0][0].visible, false);
        assert_eq!(map[0][1].visible, true);
        assert_eq!(map[6][6].visible, true);

        assert_eq!(map[0][0].zone_index, None);
        assert_eq!(map[1][1].zone_index, Some(65));
        assert_eq!(map[1][2].zone_index, Some(65));
    }
}
