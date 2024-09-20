// use num_enum::{IntoPrimitive, TryFromPrimitive};
// use serde::{Deserialize, Serialize};

// #[derive(
//     Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
// )]
// #[repr(u64)]
// #[serde(into = "u64", try_from = "u64")]
// pub enum PacketType {
//     Unknown = 0,
//     Ping = 17,
//     Pong = 18,

//     TankMovement = 11,
//     TankRotation = 12,
//     TankShoot = 13,

//     GameState = 50,
//     LobbyData = 31,
//     // LobbyDeleted = 32,
//     Ready = 102,
//     GameEnded = 103,
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PacketType {
    Unknown,
    Ping,
    Pong,

    TankMovement,
    TankRotation,
    TankShoot,

    GameStart,
    GameState,
    LobbyData,
    // LobbyDeleted,
    Ready,
    GameEnded,
}
