use crate::ws_client::packet::packets::game_state::{tile::tile::TilePayload, zone::Zone};
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor)]
pub struct RawMap {
    pub tiles: Vec<Vec<Vec<TilePayload>>>,
    pub zones: Vec<Zone>,
    pub visibility: Vec<String>,
}
