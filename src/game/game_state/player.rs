use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub nickname: String,
    pub color: u64,
    pub ping: Option<u64>,
    pub score: Option<u64>,
    pub ticks_to_regen: Option<u64>,
}
