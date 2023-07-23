#[allow(unused)]
mod integer_representation
{
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    use crate::replay::Mods;
    type IntRep = u8;
    type Flags = Mods;

    pub fn serialize<S>(date: &Flags, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        date.bits().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Flags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: IntRep = IntRep::deserialize(deserializer)?;
        Mods::from_bits(raw.into()).ok_or(serde::de::Error::custom(format!(
            "Unexpected flags value {}",
            raw
        )))
    }
}
