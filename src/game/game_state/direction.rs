use num_enum::TryFromPrimitive;
use serde::{Deserialize, Deserializer};

#[derive(Debug, TryFromPrimitive, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Direction::try_from(value).map_err(serde::de::Error::custom)
    }
}
