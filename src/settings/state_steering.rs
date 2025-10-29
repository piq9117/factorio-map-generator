use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct StateSteeringSettings {
    pub radius: Option<f32>,
    pub separation_factor: Option<f32>,
    pub separation_force: Option<f32>,
    pub force_unit_fuzzy_goto_behavior: bool,
}

impl Serialize for StateSteeringSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StateSteerSettigs", 4)?;
        s.serialize_field("radius", &self.radius.unwrap_or(0.0))?;
        s.serialize_field("separation_factor", &self.separation_factor.unwrap_or(0.0))?;
        s.serialize_field("separation_force", &self.separation_force.unwrap_or(0.0))?;
        s.serialize_field(
            "force_unit_fuzzy_goto_behavior",
            &self.force_unit_fuzzy_goto_behavior,
        )?;
        s.end()
    }
}
