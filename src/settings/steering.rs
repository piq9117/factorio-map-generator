use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::settings::state_steering::StateSteeringSettings;

pub struct SteeringSettings {
    pub default: StateSteeringSettings,
    pub moving: StateSteeringSettings,
}

impl Serialize for SteeringSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SteeringSettings", 2)?;
        s.serialize_field("default", &self.default)?;
        s.serialize_field("moving", &self.moving)?;
        s.end()
    }
}
