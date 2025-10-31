use serde::de;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone)]
pub struct EnemyEvolutionSettings {
    pub enabled: bool,
    pub time_factor: Option<f32>,
    pub destroy_factor: Option<f32>,
    pub pollution_factor: Option<f32>,
}

impl Default for EnemyEvolutionSettings {
    fn default() -> Self {
        EnemyEvolutionSettings {
            enabled: false,
            time_factor: Some(0.0),
            destroy_factor: Some(0.0),
            pollution_factor: Some(0.0),
        }
    }
}

impl Serialize for EnemyEvolutionSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EnemyEvolutionSettings", 4)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field("time_factor", &self.time_factor.unwrap_or_default())?;
        s.serialize_field("destroy_factor", &self.destroy_factor.unwrap_or_default())?;
        s.serialize_field(
            "pollution_factor",
            &self.pollution_factor.unwrap_or_default(),
        )?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for EnemyEvolutionSettings {
    fn deserialize<D>(deserializer: D) -> Result<EnemyEvolutionSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EnemyEvolutionSettingsVisitor;
        impl<'de> Visitor<'de> for EnemyEvolutionSettingsVisitor {
            type Value = EnemyEvolutionSettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("EnemyEvolutionSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut enabled: Option<bool> = None;
                let mut time_factor: Option<f32> = Some(0.0);
                let mut destroy_factor: Option<f32> = Some(0.0);
                let mut pollution_factor: Option<f32> = Some(0.0);

                while let Some(key) = map.next_key()? {
                    match key {
                        "enabled" => enabled = Some(map.next_value()?),
                        "time_factor" => time_factor = Some(map.next_value()?),
                        "destroy_factor" => destroy_factor = Some(map.next_value()?),
                        "pollution_factor" => pollution_factor = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let enabled = enabled.ok_or_else(|| de::Error::missing_field("enabled"))?;

                Ok(EnemyEvolutionSettings {
                    enabled: enabled,
                    time_factor: time_factor,
                    destroy_factor: destroy_factor,
                    pollution_factor: pollution_factor,
                })
            }
        }
        deserializer.deserialize_map(EnemyEvolutionSettingsVisitor)
    }
}
