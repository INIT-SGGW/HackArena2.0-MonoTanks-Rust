use super::{direction::Direction, turret::Turret};
use crate::ws_client::packet::packets::game_state::tile::item::ItemType;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Represents a tank in the game.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Tank {
    /// The current direction the tank is facing.
    pub direction: Direction,

    /// The health of the tank. This field is only Some for your own tank.
    pub health: Option<i64>,

    /// The unique identifier of the owner of the tank.
    /// This is a string that corresponds to a player's ID.
    pub owner_id: String,

    /// The turret attached to the tank.
    pub turret: Turret,

    /// The secondary item the tank is holding. This field is None for other tanks.
    /// It can also be None for your own tank if you have no secondary item.
    pub secondary_item: Option<ItemType>,
}
