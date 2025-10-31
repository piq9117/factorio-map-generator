use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

#[derive(Debug, Clone)]
pub struct AsteroidSettings {
    pub spawning_rate: Option<f32>,
    pub max_ray_portals_expanded_per_tick: Option<u32>,
}

impl Default for AsteroidSettings {
    fn default() -> Self {
        AsteroidSettings {
            spawning_rate: Some(0.0),
            max_ray_portals_expanded_per_tick: Some(0),
        }
    }
}

impl Serialize for AsteroidSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AsteroidSettings", 2)?;
        s.serialize_field("spawning_rate", &self.spawning_rate.unwrap_or_default())?;
        s.serialize_field(
            "max_ray_portals_expanded_per_tick",
            &self.max_ray_portals_expanded_per_tick.unwrap_or_default(),
        )?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for AsteroidSettings {
    fn deserialize<D>(deserializer: D) -> Result<AsteroidSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AsteroidSettingsVisitor;
        impl<'de> Visitor<'de> for AsteroidSettingsVisitor {
            type Value = AsteroidSettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("AsteroidSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut spawning_rate: Option<f32> = None;
                let mut max_ray_portals_expanded_per_tick: Option<u32> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "spawning_rate" => spawning_rate = Some(map.next_value()?),
                        "max_ray_portals_expanded_per_tick" => {
                            max_ray_portals_expanded_per_tick = Some(map.next_value()?)
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(AsteroidSettings {
                    spawning_rate: spawning_rate,
                    max_ray_portals_expanded_per_tick: max_ray_portals_expanded_per_tick,
                })
            }
        }
        deserializer.deserialize_map(AsteroidSettingsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_asteroids_settings() {
        insta::assert_json_snapshot!(AsteroidSettings::default());
    }
}
