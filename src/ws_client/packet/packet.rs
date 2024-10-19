use super::packets::agent_response::ability_type::AbilityType;
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
    LobbyDataRequest,

    #[serde(with = "empty_payload")]
    GameStarting,

    #[serde(with = "empty_payload")]
    GameNotStarted,

    #[serde(with = "empty_payload")]
    GameInProgress,

    #[serde(with = "empty_payload")]
    GameStatusRequest,

    #[serde(with = "empty_payload")]
    ReadyToReceiveGameState,

    #[serde(with = "empty_payload")]
    GameStarted,

    GameState(RawGameState),

    #[serde(rename_all = "camelCase")]
    Movement {
        game_state_id: String,
        direction: MoveDirection,
    },

    #[serde(rename_all = "camelCase")]
    Rotation {
        game_state_id: String,
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },

    #[serde(rename_all = "camelCase")]
    AbilityUse {
        game_state_id: String,
        ability_type: AbilityType,
    },

    #[serde(rename_all = "camelCase")]
    Pass {
        game_state_id: String,
    },

    GameEnded(GameEnd),

    // Warnings
    #[serde(with = "empty_payload")]
    PlayerAlreadyMadeActionWarning,

    #[serde(with = "empty_payload")]
    MissingGameStateIdWarning,

    #[serde(with = "empty_payload")]
    SlowResponseWarning,

    #[serde(with = "empty_payload")]
    ActionIgnoredDueToDeadWarning,

    CustomWarning {
        message: String,
    },

    // Errors
    #[serde(with = "empty_payload")]
    InvalidPacketTypeError,

    #[serde(with = "empty_payload")]
    InvalidPacketUsageError,

    InvalidPayloadError {
        message: String,
    },
}

impl From<Packet> for String {
    fn from(packet: Packet) -> Self {
        serde_json::to_string(&packet).unwrap()
    }
}

impl AgentResponse {
    pub fn to_packet(self, game_state_id: String) -> Packet {
        match self {
            AgentResponse::Movement { direction } => Packet::Movement {
                game_state_id,
                direction,
            },
            AgentResponse::Rotation {
                tank_rotation,
                turret_rotation,
            } if tank_rotation.is_none() && turret_rotation.is_none() => {
                Packet::Pass { game_state_id }
            }
            AgentResponse::Rotation {
                tank_rotation,
                turret_rotation,
            } => Packet::Rotation {
                game_state_id,
                tank_rotation,
                turret_rotation,
            },
            AgentResponse::AbilityUse { ability_type } => Packet::AbilityUse {
                game_state_id,
                ability_type,
            },
            AgentResponse::Pass => Packet::Pass { game_state_id },
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
