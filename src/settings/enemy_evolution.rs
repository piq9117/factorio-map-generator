use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
pub struct EnemyEvolutionSettings {
    pub enabled: bool,
    pub time_factor: f32,
    pub destroy_factor: f32,
    pub pollution_factor: f32,
}

impl Serialize for EnemyEvolutionSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EnemyEvolutionSettings", 4)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field("time_factor", &self.time_factor)?;
        s.serialize_field("destroy_factor", &self.destroy_factor)?;
        s.serialize_field("pollution_factor", &self.pollution_factor)?;
        s.end()
    }
}
