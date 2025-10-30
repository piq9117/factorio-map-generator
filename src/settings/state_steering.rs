use serde::de;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

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

impl<'de> Deserialize<'de> for StateSteeringSettings {
    fn deserialize<D>(deserializer: D) -> Result<StateSteeringSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StateSteeringSettingsVisitor;
        impl<'de> Visitor<'de> for StateSteeringSettingsVisitor {
            type Value = StateSteeringSettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StateSteeringSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut radius: Option<f32> = Some(0.0);
                let mut separation_factor: Option<f32> = Some(0.0);
                let mut separation_force: Option<f32> = Some(0.0);
                let mut force_unit_fuzzy_goto_behavior: Option<bool> = None;

                while let Some(key) = map.next_value()? {
                    match key {
                        "radius" => radius = Some(map.next_value()?),
                        "separation_factor" => separation_factor = Some(map.next_value()?),
                        "separation_force" => separation_force = Some(map.next_value()?),
                        "force_unit_fuzzy_goto_behavior" => {
                            force_unit_fuzzy_goto_behavior = Some(map.next_value()?)
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let force_unit_fuzzy_goto_behavior = force_unit_fuzzy_goto_behavior
                    .ok_or_else(|| de::Error::missing_field("force_unit_fuzzy_goto_behavior"))?;

                Ok(StateSteeringSettings {
                    radius: radius,
                    separation_factor: separation_factor,
                    separation_force: separation_force,
                    force_unit_fuzzy_goto_behavior: force_unit_fuzzy_goto_behavior,
                })
            }
        }
        deserializer.deserialize_map(StateSteeringSettingsVisitor)
    }
}
