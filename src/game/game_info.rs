use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub broadcast_interval: u32,
    pub player_id: String,
}

impl TryFrom<Value> for GameInfo {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| e.to_string())
    }
}
