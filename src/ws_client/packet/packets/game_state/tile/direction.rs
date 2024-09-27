use derive_more::derive::IsVariant;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Represents the four cardinal directions.
#[derive(
    Debug,
    IntoPrimitive,
    TryFromPrimitive,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IsVariant,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[serde(into = "u64", try_from = "u64")]
#[repr(u64)]
pub enum Direction {
    /// Represents upward direction.
    Up = 0,

    /// Represents rightward direction.
    Right = 1,

    /// Represents downward direction.
    Down = 2,

    /// Represents leftward direction.
    Left = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        let direction = Direction::Up;
        let serialized = serde_json::to_string(&direction).unwrap();
        assert_eq!(serialized, "0");
    }

    #[test]
    fn test_deserialize() {
        let deserialized: Direction = serde_json::from_str("1").unwrap();
        assert_eq!(deserialized, Direction::Right);
    }

    #[test]
    fn test_deserialize_invalid() {
        let deserialized: Result<Direction, _> = serde_json::from_str("4");
        assert!(deserialized.is_err());
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let deserialized: Result<Direction, _> = serde_json::from_str("\"1\"");
        assert!(deserialized.is_err());
    }

    #[test]
    fn test_deserialize_invalid_type2() {
        // It should be a number
        let deserialized: Result<Direction, _> = serde_json::from_str("Up");
        assert!(deserialized.is_err());
    }
}
