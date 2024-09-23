use super::dto::game_end::GameEnd;
use super::dto::raw_game_state::RawGameState;
use super::dto::rotation::Rotation;
use super::dto::{lobby_data::LobbyData, move_direction::MoveDirection};
use crate::ws_client::packet::empty_payload;
use serde::{Deserialize, Serialize};
use subenum::subenum;

#[subenum(AgentResponse)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "payload")]
pub enum Packet {
    #[serde(with = "empty_payload")]
    Ping,
    #[serde(with = "empty_payload")]
    Pong,

    LobbyData(LobbyData),

    #[serde(with = "empty_payload")]
    GameStart,

    GameState(RawGameState),
    #[subenum(AgentResponse)]
    TankMovement {
        direction: MoveDirection,
    },
    #[subenum(AgentResponse)]
    #[serde(rename_all = "camelCase")]
    TankRotation {
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },
    #[subenum(AgentResponse)]
    #[serde(with = "empty_payload")]
    TankShoot,

    GameEnd(GameEnd),
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
