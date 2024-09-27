use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

/// Represents a zone in the game world.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    /// The unique index of the zone.
    pub index: u8,
    /// The x-coordinate of the left side of the zone.
    pub x: u64,
    /// The y-coordinate of the top side of the zone.
    pub y: u64,
    /// The width of the zone.
    pub width: u64,
    /// The height of the zone.
    pub height: u64,
    /// The current status of the zone.
    pub status: ZoneStatus,
}

/// Represents the status of a zone.
///
/// The status of a zone can be neutral, being captured, captured, being contested, or being retaken.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, IsVariant)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ZoneStatus {
    /// The zone is neutral and not controlled by any player.
    Neutral,

    /// The zone is being captured by a player.
    BeingCaptured(BeingCapturedStatus),

    /// The zone has been captured by a player and he receives points.
    Captured(CapturedStatus),

    /// The zone is being contested by players.
    BeingContested(BeingContestedStatus),

    /// The zone is being retaken by another player.
    BeingRetaken(BeingRetakenStatus),
}

/// Represents the status of a zone being captured.
///
/// This struct contains information about the player capturing the zone and the remaining time
/// until the zone is fully captured.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BeingCapturedStatus {
    /// The remaining ticks until the zone is captured.
    pub remaining_ticks: u64,
    /// The ID of the player capturing the zone.
    pub player_id: String,
}

/// Represents the status of a zone that has been captured.
///
/// This struct contains information about the player who captured the zone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CapturedStatus {
    /// The ID of the player who captured the zone.
    pub player_id: String,
}

/// Represents the status of a zone being contested.
///
/// This struct contains information about the player who captured the zone, if any.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BeingContestedStatus {
    /// The ID of the player who captured the zone, if any.
    pub captured_by_id: Option<String>,
}

/// Represents the status of a zone being retaken.
///
/// This struct contains information about the remaining time until the zone is retaken,
/// the player who previously captured the zone, and the player retaking the zone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BeingRetakenStatus {
    /// The remaining ticks until the zone is retaken.
    pub remaining_ticks: u64,
    /// The ID of the player who previously captured the zone.
    pub captured_by_id: String,
    /// The ID of the player retaking the zone.
    pub retaken_by_id: String,
}
