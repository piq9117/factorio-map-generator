use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct AsteriodSettings {
    pub spawning_rate: f32,
    pub max_ray_portals_expanded_per_tick: u32,
}

impl Serialize for AsteriodSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AsteriodSettings", 2)?;
        s.serialize_field("spawning_rate", &self.spawning_rate)?;
        s.serialize_field(
            "max_ray_portals_expanded_per_tick",
            &self.max_ray_portals_expanded_per_tick,
        )?;
        s.end()
    }
}
