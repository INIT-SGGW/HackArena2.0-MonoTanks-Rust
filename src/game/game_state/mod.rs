use player::Player;
use tile::Tile;
use zone::Zone;

pub mod bullet;
pub mod direction;
pub mod parser;
pub mod player;
pub mod tank;
pub mod tile;
pub mod turret;
pub mod zone;

pub struct GameState {
    raw_payload: serde_json::Value,
    map: Vec<Vec<Tile>>,
    players: Vec<Player>,
    tick: u64,
    zones: Vec<Zone>,
}

impl GameState {
    pub fn new(
        payload: serde_json::Value,
        tick: u64,
        players: Vec<Player>,
        map: Vec<Vec<Tile>>,
        zones: Vec<Zone>,
    ) -> Self {
        GameState {
            raw_payload: payload,
            map,
            tick,
            players,
            zones,
        }
    }

    pub fn get_raw_payload(&self) -> &serde_json::Value {
        &self.raw_payload
    }

    pub fn get_map(&self) -> &Vec<Vec<Tile>> {
        &self.map
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_time(&self) -> u64 {
        self.tick
    }

    pub fn get_zones(&self) -> &Vec<Zone> {
        &self.zones
    }
}
