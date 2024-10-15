use super::{player::Player, raw_game_state::RawGameState, tile::tile::Tile, zone::Zone};

use derive_more::Constructor;

/// Represents the current state of the game.
#[derive(Constructor, Clone, Debug, PartialEq)]
pub struct GameState {
    /// A 2D vector representing the game map, where each element is a `Tile`.
    pub map: Vec<Vec<Tile>>,

    /// A vector of `Player` objects representing all the players in the game.
    pub players: Vec<Player>,

    /// The current tick of the game.
    pub tick: u64,

    /// A vector of `Zone` objects representing different zones in the game.
    pub zones: Vec<Zone>,
}

impl From<RawGameState> for GameState {
    fn from(raw_game_state: RawGameState) -> Self {
        let x = raw_game_state.map.tiles.len();
        let y = raw_game_state.map.tiles[0].len();

        let mut map = vec![vec![Tile::new(false, None, vec![]); x]; y];

        // Payload
        for (x, column) in raw_game_state.map.tiles.iter().enumerate() {
            for (y, row) in column.iter().enumerate() {
                if !row.is_empty() {
                    map[y][x].entities = row.clone();
                }
            }
        }

        // Visibility
        for (y, row) in raw_game_state.map.visibility.iter().enumerate() {
            for (x, column) in row.chars().enumerate() {
                map[y][x].visible = column == '1';
            }
        }

        // Zone index
        for zone in raw_game_state.map.zones.iter() {
            for y in zone.y..zone.y + zone.height {
                for x in zone.x..zone.x + zone.width {
                    map[y as usize][x as usize].zone_index = Some(zone.index);
                }
            }
        }

        GameState::new(
            map,
            raw_game_state.players,
            raw_game_state.tick,
            raw_game_state.map.zones,
        )
    }
}
