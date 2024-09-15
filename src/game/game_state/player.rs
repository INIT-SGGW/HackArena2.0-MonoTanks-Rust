use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub nickname: String,
    pub color: u64,
    pub ping: u64,
    pub score: Option<u64>,
}
