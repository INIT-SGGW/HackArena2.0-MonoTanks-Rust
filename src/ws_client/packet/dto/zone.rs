use derive_more::derive::{Constructor, IsVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    pub index: u8,
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
    pub status: ZoneStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, IsVariant)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ZoneStatus {
    Neutral,

    #[serde(rename_all = "camelCase")]
    BeingCaptured {
        remaining_ticks: u64,
        player_id: String,
    },

    #[serde(rename_all = "camelCase")]
    Captured {
        player_id: String,
    },

    #[serde(rename_all = "camelCase")]
    BeingContested {
        captured_by_id: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    BeingRetaken {
        remaining_ticks: u64,
        captured_by_id: String,
        retaken_by_id: String,
    },
}
