use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

use crate::settings::state_steering::StateSteeringSettings;

#[derive(Debug, Clone)]
pub struct SteeringSettings {
    pub default: Option<StateSteeringSettings>,
    pub moving: Option<StateSteeringSettings>,
}

impl Default for SteeringSettings {
    fn default() -> Self {
        SteeringSettings {
            default: Default::default(),
            moving: Default::default(),
        }
    }
}

impl Serialize for SteeringSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SteeringSettings", 2)?;
        s.serialize_field("default", &self.default.clone().unwrap_or_default())?;
        s.serialize_field("moving", &self.moving.clone().unwrap_or_default())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for SteeringSettings {
    fn deserialize<D>(deserializer: D) -> Result<SteeringSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SteeringSettingsVisitor;
        impl<'de> Visitor<'de> for SteeringSettingsVisitor {
            type Value = SteeringSettings;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("SteeringSettings")
            }
            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut default: Option<StateSteeringSettings> = None;
                let mut moving: Option<StateSteeringSettings> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "default" => default = Some(map.next_value()?),
                        "moving" => moving = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(SteeringSettings {
                    default: default,
                    moving: moving,
                })
            }
        }
        deserializer.deserialize_map(SteeringSettingsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_steering_settings() {
        insta::assert_yaml_snapshot!(SteeringSettings::default());
    }
}
