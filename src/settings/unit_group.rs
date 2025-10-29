use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct UnitGroupSettings {
    pub min_group_gathering_time: u32,
    pub max_group_gathering_time: u32,
    pub max_wait_time_for_late_members: u32,
    pub max_group_radius: f32,
    pub min_group_radius: f32,
    pub max_member_speedup_when_behind: f32,
    pub max_member_slowdown_when_ahead: f32,
    pub max_group_slowdown_factor: f32,
    pub max_group_member_fallback_factor: f32,
    pub member_disown_distance: f32,
    pub tick_tolerance_when_members_arrives: u32,
    pub max_gathering_unit_groups: u32,
    pub max_unit_group_size: u32,
}

impl Serialize for UnitGroupSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("UnitGroupSettings", 13)?;
        s.serialize_field("min_group_gathering_time", &self.min_group_gathering_time)?;
        s.serialize_field("max_group_gathering_time", &self.max_group_gathering_time)?;
        s.serialize_field(
            "max_wait_time_for_late_members",
            &self.max_wait_time_for_late_members,
        )?;
        s.serialize_field("max_group_radius", &self.max_group_radius)?;
        s.serialize_field("min_group_radius", &self.min_group_radius)?;
        s.serialize_field(
            "max_member_speedup_when_behind",
            &self.max_member_speedup_when_behind,
        )?;
        s.serialize_field(
            "max_member_slowdown_when_ahead",
            &self.max_member_slowdown_when_ahead,
        )?;
        s.serialize_field("max_group_slowdown_factor", &self.max_group_slowdown_factor)?;
        s.serialize_field(
            "max_group_member_fallback_factor",
            &self.max_group_member_fallback_factor,
        )?;
        s.serialize_field("member_disown_distance", &self.member_disown_distance)?;
        s.serialize_field(
            "tick_tolerance_when_members_arrives",
            &self.tick_tolerance_when_members_arrives,
        )?;
        s.serialize_field("max_gathering_unit_groups", &self.max_gathering_unit_groups)?;
        s.serialize_field("max_unit_group_size", &self.max_unit_group_size)?;
        s.end()
    }
}
