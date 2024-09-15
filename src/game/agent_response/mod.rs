pub mod move_direction;
pub mod rotation;

use crate::ws_client::packet::packet_type_enum::PacketType;
use move_direction::MoveDirection;
use rotation::Rotation;
use serde::{ser::SerializeStruct, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentResponsePayload {
    Move {
        direction: MoveDirection,
    },
    Rotation {
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },
    Shoot,
}

// Impl from AgentResponsePayload to PacketType
impl From<AgentResponsePayload> for PacketType {
    fn from(payload: AgentResponsePayload) -> Self {
        match payload {
            AgentResponsePayload::Move { .. } => PacketType::TankMovement,
            AgentResponsePayload::Rotation { .. } => PacketType::TankRotation,
            AgentResponsePayload::Shoot => PacketType::TankShoot,
        }
    }
}

// Impl Serialize for AgentResponsePayload
impl Serialize for AgentResponsePayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AgentResponsePayload::Move { direction } => {
                let mut state = serializer.serialize_struct("AgentResponsePayload", 2)?;
                state.serialize_field("type", &PacketType::TankMovement)?;
                state.serialize_field("payload", &json!({"direction": *direction as u64}))?;
                state.end()
            }
            AgentResponsePayload::Rotation {
                tank_rotation,
                turret_rotation,
            } => {
                let mut state = serializer.serialize_struct("AgentResponsePayload", 2)?;
                state.serialize_field("type", &PacketType::TankRotation)?;
                state.serialize_field(
                    "payload",
                    &json!({
                        "tankRotation": tank_rotation.map(|r| r as u64),
                        "turretRotation": turret_rotation.map(|r| r as u64),
                    }),
                )?;
                state.end()
            }
            AgentResponsePayload::Shoot => {
                let mut state = serializer.serialize_struct("AgentResponsePayload", 2)?;
                state.serialize_field("type", &PacketType::TankShoot)?;
                state.serialize_field("payload", &json!({}))?;
                state.end()
            }
        }
    }
}
