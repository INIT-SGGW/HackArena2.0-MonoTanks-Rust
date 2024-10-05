use super::packets::agent_response::agent_response::AgentResponse;
use super::packets::agent_response::move_direction::MoveDirection;
use super::packets::game_state::raw_game_state::RawGameState;
use super::packets::lobby_data::LobbyData;
use super::packets::{agent_response::rotation::Rotation, game_end::game_end::GameEnd};
use crate::ws_client::packet::empty_payload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "payload")]
pub enum Packet {
    #[serde(with = "empty_payload")]
    Ping,
    #[serde(with = "empty_payload")]
    Pong,

    #[serde(with = "empty_payload")]
    ConnectionAccepted,
    ConnectionRejected {
        reason: String,
    },

    LobbyData(LobbyData),

    #[serde(with = "empty_payload")]
    LobbyDeleted,

    #[serde(with = "empty_payload")]
    GameStart,

    GameState(RawGameState),

    #[serde(rename_all = "camelCase")]
    TankMovement {
        game_state_id: String,
        direction: MoveDirection,
    },

    #[serde(rename_all = "camelCase")]
    TankRotation {
        game_state_id: String,
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },

    #[serde(rename_all = "camelCase")]
    TankShoot {
        game_state_id: String,
    },

    GameEnd(GameEnd),

    // Warnings
    #[serde(with = "empty_payload")]
    PlayerAlreadyMadeActionWarning,

    #[serde(with = "empty_payload")]
    MissingGameStateIdWarning,

    #[serde(with = "empty_payload")]
    SlowResponseWarning,

    // Errors
    #[serde(with = "empty_payload")]
    InvalidPacketTypeError,

    #[serde(with = "empty_payload")]
    InvalidPacketUsageError,
}

impl From<Packet> for String {
    fn from(packet: Packet) -> Self {
        serde_json::to_string(&packet).unwrap()
    }
}

impl AgentResponse {
    pub fn to_packet(self, game_state_id: String) -> Packet {
        match self {
            AgentResponse::TankMovement { direction } => Packet::TankMovement {
                game_state_id,
                direction,
            },
            AgentResponse::TankRotation {
                tank_rotation,
                turret_rotation,
            } => Packet::TankRotation {
                game_state_id,
                tank_rotation,
                turret_rotation,
            },
            AgentResponse::TankShoot => Packet::TankShoot { game_state_id },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_packet_type_ping() {
        let packet = Packet::Ping;
        let serialized = serde_json::to_string(&packet).unwrap();
        let expected = json!({"type": "ping", "payload": {}}).to_string();

        // Deserialize both JSON strings into `serde_json::Value`
        let serialized_value: Value = serde_json::from_str(&serialized).unwrap();
        let expected_value: Value = serde_json::from_str(&expected).unwrap();

        // Compare the `Value` objects
        assert_eq!(serialized_value, expected_value);

        let deserialized: Packet = serde_json::from_value(expected_value).unwrap();
        assert_eq!(deserialized, packet);
    }
}
