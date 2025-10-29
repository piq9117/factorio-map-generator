use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct DifficultySettings {
    pub technology_price_multiplier: Option<f32>,
    pub spoil_time_modifier: Option<f32>,
}

impl Serialize for DifficultySettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DifficultySettings", 2)?;
        s.serialize_field(
            "technology_price_multiplier",
            &self.technology_price_multiplier.unwrap_or(0.001),
        )?;
        s.serialize_field(
            "spoil_time_modifier",
            &self.spoil_time_modifier.unwrap_or(0.0),
        )?;
        s.end()
    }
}
