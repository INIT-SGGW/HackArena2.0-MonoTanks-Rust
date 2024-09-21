use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

use crate::ws_client::packet::dto::{tile::TilePayload, zone::Zone};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Constructor)]
pub struct Map {
    pub tiles: Vec<Vec<Vec<TilePayload>>>,
    pub zones: Vec<Zone>,
    pub visibility: Vec<String>,
}
