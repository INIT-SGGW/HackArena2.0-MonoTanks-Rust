use derive_more::derive::IsVariant;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, TryFromPrimitive, Clone, Copy, PartialEq, Eq, Hash, Serialize, IsVariant)]
#[repr(u64)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive)]
// #[serde(rename_all = "camelCase")]
// #[serde(into = "u64")]
// #[repr(u64)]

// TODO: Can we use a macro to generate this?
impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Direction::try_from(value).map_err(serde::de::Error::custom)
    }
}
