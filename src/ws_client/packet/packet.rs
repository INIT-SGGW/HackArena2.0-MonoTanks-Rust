use super::packet_type_enum::PacketType;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    #[serde(rename = "type")]
    pub packet_type: PacketType,
    pub payload: serde_json::Value,
}

impl Packet {
    pub fn construct_pong_packet() -> Packet {
        Packet {
            packet_type: PacketType::Pong,
            payload: json!({}),
        }
    }
}
