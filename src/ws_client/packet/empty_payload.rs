use serde::de::{MapAccess, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;

pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let map = serializer.serialize_map(Some(0))?;
    map.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    struct EmptyPayloadVisitor;

    impl<'de> Visitor<'de> for EmptyPayloadVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an empty object")
        }

        fn visit_map<M>(self, mut access: M) -> Result<(), M::Error>
        where
            M: MapAccess<'de>,
        {
            if access.next_entry::<String, ()>()?.is_some() {
                return Err(serde::de::Error::custom("expected empty object"));
            }
            Ok(())
        }
    }

    deserializer.deserialize_map(EmptyPayloadVisitor)
}
