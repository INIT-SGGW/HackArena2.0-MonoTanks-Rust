use derive_more::derive::IsVariant;
use serde::{Deserialize, Serialize};

/// Represents the four cardinal directions.
#[derive(
    Debug,
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
pub enum Direction {
    /// Represents upward direction.
    Up,

    /// Represents rightward direction.
    Right,

    /// Represents downward direction.
    Down,

    /// Represents leftward direction.
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        let direction = Direction::Up;
        let serialized = serde_json::to_string(&direction).unwrap();
        assert_eq!(serialized, "\"up\"");
    }

    #[test]
    fn test_deserialize() {
        let deserialized: Direction = serde_json::from_str("\"right\"").unwrap();
        assert_eq!(deserialized, Direction::Right);
    }

    #[test]
    fn test_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<Direction>("\"up\"").unwrap(), Direction::Up);
        assert_eq!(serde_json::from_str::<Direction>("\"right\"").unwrap(), Direction::Right);
        assert_eq!(serde_json::from_str::<Direction>("\"down\"").unwrap(), Direction::Down);
        assert_eq!(serde_json::from_str::<Direction>("\"left\"").unwrap(), Direction::Left);
    }

    #[test]
    fn test_deserialize_invalid() {
        let deserialized: Result<Direction, _> = serde_json::from_str("\"invalid\"");
        assert!(deserialized.is_err());
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let deserialized: Result<Direction, _> = serde_json::from_str("1");
        assert!(deserialized.is_err());
    }
}
