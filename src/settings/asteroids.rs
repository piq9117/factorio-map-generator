use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct AsteroidSettings {
    pub spawning_rate: Option<f32>,
    pub max_ray_portals_expanded_per_tick: Option<u32>,
}

impl Serialize for AsteroidSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AsteroidSettings", 2)?;
        s.serialize_field("spawning_rate", &self.spawning_rate.unwrap_or(0.0))?;
        s.serialize_field(
            "max_ray_portals_expanded_per_tick",
            &self.max_ray_portals_expanded_per_tick.unwrap_or(0),
        )?;
        s.end()
    }
}
