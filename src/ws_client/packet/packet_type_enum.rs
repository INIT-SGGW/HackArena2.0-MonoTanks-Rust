use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u64)]
#[serde(into = "u64", try_from = "u64")]
pub enum PacketType {
    Unknown = 0,
    Ping = 1,
    Pong = 2,

    TankMovement = 11,
    TankRotation = 12,
    TankShoot = 13,

    GameState = 21,
    LobbyData = 31,
    // LobbyDeleted = 32,
    Ready = 102,
    GameEnded = 103,
}
