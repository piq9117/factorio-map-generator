use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct EnemyExpansionSettings {
    pub enabled: bool,
    pub max_expansion_distance: Option<u32>,
    pub friendly_base_influence_radius: Option<u32>,
    pub enemy_building_influence_radius: Option<u32>,
    pub building_coefficient: Option<f32>,
    pub other_base_coefficient: Option<f32>,
    pub neighbouring_base_chunk_coefficient: Option<f32>,
    pub max_colliding_tiles_coefficient: Option<f32>,
    pub settler_group_min_size: Option<u32>,
    pub settler_group_max_size: Option<u32>,
    pub min_expansion_cooldown: Option<u32>,
    pub max_expansion_cooldown: Option<u32>,
}

impl Serialize for EnemyExpansionSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EnemyExpansionSettings", 12)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field("max_expansion_distance", &self.max_expansion_distance)?;
        s.serialize_field(
            "friendly_base_influence_radius",
            &self.friendly_base_influence_radius,
        )?;
        s.serialize_field(
            "enemy_building_influence_radius",
            &self.enemy_building_influence_radius,
        )?;
        s.serialize_field("building_coefficient", &self.building_coefficient)?;
        s.serialize_field("other_base_coefficient", &self.other_base_coefficient)?;
        s.serialize_field(
            "neighbouring_base_chunk_coefficient",
            &self.neighbouring_base_chunk_coefficient,
        )?;
        s.serialize_field(
            "max_colliding_tiles_coefficient",
            &self.max_colliding_tiles_coefficient,
        )?;
        s.serialize_field("settler_group_min_size", &self.settler_group_min_size)?;
        s.serialize_field("settler_group_max_size", &self.settler_group_max_size)?;
        s.serialize_field("min_expansion_cooldown", &self.min_expansion_cooldown)?;
        s.serialize_field("max_expansion_cooldown", &self.max_expansion_cooldown)?;
        s.end()
    }
}
