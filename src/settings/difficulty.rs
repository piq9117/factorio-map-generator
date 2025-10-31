use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone)]
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

impl Default for DifficultySettings {
    fn default() -> Self {
        DifficultySettings {
            technology_price_multiplier: Some(0.001),
            spoil_time_modifier: Some(0.0),
        }
    }
}

impl<'de> Deserialize<'de> for DifficultySettings {
    fn deserialize<D>(deserializer: D) -> Result<DifficultySettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DifficultySettingsVisitor;
        impl<'de> Visitor<'de> for DifficultySettingsVisitor {
            type Value = DifficultySettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DifficultySettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut technology_price_multiplier: Option<f32> = None;
                let mut spoil_time_modifier: Option<f32> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "technology_price_multiplier" => {
                            technology_price_multiplier = Some(map.next_value()?)
                        }
                        "spoil_time_modifier" => spoil_time_modifier = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(DifficultySettings {
                    technology_price_multiplier: technology_price_multiplier,
                    spoil_time_modifier: spoil_time_modifier,
                })
            }
        }
        deserializer.deserialize_map(DifficultySettingsVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_difficulty_settings() {
        insta::assert_yaml_snapshot!(DifficultySettings::default());
    }
}
