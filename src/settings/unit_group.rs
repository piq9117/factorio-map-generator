use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

#[derive(Debug, Clone)]
pub struct UnitGroupSettings {
    pub min_group_gathering_time: Option<u32>,
    pub max_group_gathering_time: Option<u32>,
    pub max_wait_time_for_late_members: Option<u32>,
    pub max_group_radius: Option<f32>,
    pub min_group_radius: Option<f32>,
    pub max_member_speedup_when_behind: Option<f32>,
    pub max_member_slowdown_when_ahead: Option<f32>,
    pub max_group_slowdown_factor: Option<f32>,
    pub max_group_member_fallback_factor: Option<f32>,
    pub member_disown_distance: Option<f32>,
    pub tick_tolerance_when_member_arrives: Option<u32>,
    pub max_gathering_unit_groups: Option<u32>,
    pub max_unit_group_size: Option<u32>,
}

impl Default for UnitGroupSettings {
    fn default() -> Self {
        UnitGroupSettings {
            min_group_gathering_time: Some(0),
            max_group_gathering_time: Some(0),
            max_wait_time_for_late_members: Some(0),
            max_group_radius: Some(0.0),
            min_group_radius: Some(0.0),
            max_member_speedup_when_behind: Some(0.0),
            max_member_slowdown_when_ahead: Some(0.0),
            max_group_slowdown_factor: Some(0.0),
            max_group_member_fallback_factor: Some(0.0),
            member_disown_distance: Some(0.0),
            tick_tolerance_when_member_arrives: Some(0),
            max_gathering_unit_groups: Some(0),
            max_unit_group_size: Some(0),
        }
    }
}

impl Serialize for UnitGroupSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("UnitGroupSettings", 13)?;
        s.serialize_field(
            "min_group_gathering_time",
            &self.min_group_gathering_time.unwrap_or(0),
        )?;
        s.serialize_field(
            "max_group_gathering_time",
            &self.max_group_gathering_time.unwrap_or(0),
        )?;
        s.serialize_field(
            "max_wait_time_for_late_members",
            &self.max_wait_time_for_late_members.unwrap_or(0),
        )?;
        s.serialize_field("max_group_radius", &self.max_group_radius.unwrap_or(0.0))?;
        s.serialize_field("min_group_radius", &self.min_group_radius.unwrap_or(0.0))?;
        s.serialize_field(
            "max_member_speedup_when_behind",
            &self.max_member_speedup_when_behind.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_member_slowdown_when_ahead",
            &self.max_member_slowdown_when_ahead.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_group_slowdown_factor",
            &self.max_group_slowdown_factor.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_group_member_fallback_factor",
            &self.max_group_member_fallback_factor.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "member_disown_distance",
            &self.member_disown_distance.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "tick_tolerance_when_member_arrives",
            &self.tick_tolerance_when_member_arrives.unwrap_or(0),
        )?;
        s.serialize_field(
            "max_gathering_unit_groups",
            &self.max_gathering_unit_groups.unwrap_or(0),
        )?;
        s.serialize_field(
            "max_unit_group_size",
            &self.max_unit_group_size.unwrap_or(0),
        )?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for UnitGroupSettings {
    fn deserialize<D>(deserializer: D) -> Result<UnitGroupSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnitGroupSettingsVisitor;
        impl<'de> Visitor<'de> for UnitGroupSettingsVisitor {
            type Value = UnitGroupSettings;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("UnitGroupSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut min_group_gathering_time: Option<u32> = None;
                let mut max_group_gathering_time: Option<u32> = None;
                let mut max_wait_time_for_late_members: Option<u32> = None;
                let mut max_group_radius: Option<f32> = None;
                let mut min_group_radius: Option<f32> = None;
                let mut max_member_speedup_when_behind: Option<f32> = None;
                let mut max_member_slowdown_when_ahead: Option<f32> = None;
                let mut max_group_slowdown_factor: Option<f32> = None;
                let mut max_group_member_fallback_factor: Option<f32> = None;
                let mut member_disown_distance: Option<f32> = None;
                let mut tick_tolerance_when_member_arrives: Option<u32> = None;
                let mut max_gathering_unit_groups: Option<u32> = None;
                let mut max_unit_group_size: Option<u32> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "min_group_gathering_time" => {
                            min_group_gathering_time = Some(map.next_value()?)
                        }
                        "max_group_gathering_time" => {
                            max_group_gathering_time = Some(map.next_value()?)
                        }
                        "max_wait_time_for_late_members" => {
                            max_wait_time_for_late_members = Some(map.next_value()?)
                        }
                        "max_group_radius" => max_group_radius = Some(map.next_value()?),
                        "min_group_radius" => min_group_radius = Some(map.next_value()?),
                        "max_member_speedup_when_behind" => {
                            max_member_speedup_when_behind = Some(map.next_value()?)
                        }
                        "max_member_slowdown_when_ahead" => {
                            max_member_slowdown_when_ahead = Some(map.next_value()?)
                        }
                        "max_group_slowdown_factor" => {
                            max_group_slowdown_factor = Some(map.next_value()?)
                        }
                        "max_group_member_fallback_factor" => {
                            max_group_member_fallback_factor = Some(map.next_value()?)
                        }
                        "member_disown_distance" => {
                            member_disown_distance = Some(map.next_value()?)
                        }
                        "tick_tolerance_when_member_arrives" => {
                            tick_tolerance_when_member_arrives = Some(map.next_value()?)
                        }
                        "max_gathering_unit_groups" => {
                            max_gathering_unit_groups = Some(map.next_value()?)
                        }
                        "max_unit_group_size" => max_unit_group_size = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(UnitGroupSettings {
                    min_group_gathering_time: min_group_gathering_time,
                    max_group_gathering_time: max_group_gathering_time,
                    max_wait_time_for_late_members: max_wait_time_for_late_members,
                    max_group_radius: max_group_radius,
                    min_group_radius: min_group_radius,
                    max_member_speedup_when_behind: max_member_speedup_when_behind,
                    max_member_slowdown_when_ahead: max_member_slowdown_when_ahead,
                    max_group_slowdown_factor: max_group_slowdown_factor,
                    max_group_member_fallback_factor: max_group_member_fallback_factor,
                    member_disown_distance: member_disown_distance,
                    tick_tolerance_when_member_arrives: tick_tolerance_when_member_arrives,
                    max_gathering_unit_groups: max_gathering_unit_groups,
                    max_unit_group_size: max_unit_group_size,
                })
            }
        }
        deserializer.deserialize_map(UnitGroupSettingsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_unit_group_settings() {
        insta::assert_yaml_snapshot!(UnitGroupSettings::default());
    }
}
