use serde::de;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub struct EnemyExpansionSettings {
    pub enabled: bool,
    pub max_expansion_distance: Option<u32>,
    pub friendly_base_influence_radius: Option<u32>,
    pub enemy_building_influence_radius: Option<u32>,
    pub building_coefficient: Option<f32>,
    pub other_base_coefficient: Option<f32>,
    pub neighbouring_chunk_coefficient: Option<f32>,
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
        s.serialize_field(
            "max_expansion_distance",
            &self.max_expansion_distance.unwrap_or(0),
        )?;
        s.serialize_field(
            "friendly_base_influence_radius",
            &self.friendly_base_influence_radius.unwrap_or(0),
        )?;
        s.serialize_field(
            "enemy_building_influence_radius",
            &self.enemy_building_influence_radius.unwrap_or(0),
        )?;
        s.serialize_field(
            "building_coefficient",
            &self.building_coefficient.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "other_base_coefficient",
            &self.other_base_coefficient.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "neighbouring_chunk_coefficient",
            &self.neighbouring_chunk_coefficient.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "neighbouring_base_chunk_coefficient",
            &self.neighbouring_base_chunk_coefficient.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_colliding_tiles_coefficient",
            &self.max_colliding_tiles_coefficient.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "settler_group_min_size",
            &self.settler_group_min_size.unwrap_or(0),
        )?;
        s.serialize_field(
            "settler_group_max_size",
            &self.settler_group_max_size.unwrap_or(0),
        )?;
        s.serialize_field(
            "min_expansion_cooldown",
            &self.min_expansion_cooldown.unwrap_or(0),
        )?;
        s.serialize_field(
            "max_expansion_cooldown",
            &self.max_expansion_cooldown.unwrap_or(0),
        )?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for EnemyExpansionSettings {
    fn deserialize<D>(deserializer: D) -> Result<EnemyExpansionSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EnemyExpansionSettingsVisitor;
        impl<'de> Visitor<'de> for EnemyExpansionSettingsVisitor {
            type Value = EnemyExpansionSettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("EnemyExpansionSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut enabled: Option<bool> = None;
                let mut max_expansion_distance: Option<u32> = Some(0);
                let mut friendly_base_influence_radius: Option<u32> = Some(0);
                let mut enemy_building_influence_radius: Option<u32> = Some(0);
                let mut building_coefficient: Option<f32> = Some(0.0);
                let mut other_base_coefficient: Option<f32> = Some(0.0);
                let mut neighbouring_chunk_coefficient: Option<f32> = Some(0.0);
                let mut neighbouring_base_chunk_coefficient: Option<f32> = Some(0.0);
                let mut max_colliding_tiles_coefficient: Option<f32> = Some(0.0);
                let mut settler_group_min_size: Option<u32> = Some(0);
                let mut settler_group_max_size: Option<u32> = Some(0);
                let mut min_expansion_cooldown: Option<u32> = Some(0);
                let mut max_expansion_cooldown: Option<u32> = Some(0);

                while let Some(key) = map.next_key()? {
                    match key {
                        "enabled" => enabled = Some(map.next_value()?),
                        "max_expansion_distance" => {
                            max_expansion_distance = Some(map.next_value()?)
                        }
                        "friendly_base_influence_radius" => {
                            friendly_base_influence_radius = Some(map.next_value()?)
                        }
                        "enemy_building_influence_radius" => {
                            enemy_building_influence_radius = Some(map.next_value()?)
                        }
                        "building_coefficient" => building_coefficient = Some(map.next_value()?),
                        "other_base_coefficient" => {
                            other_base_coefficient = Some(map.next_value()?)
                        }
                        "neighbouring_chunk_coefficient" => {
                            neighbouring_chunk_coefficient = Some(map.next_value()?)
                        }
                        "neighbouring_base_chunk_coefficient" => {
                            neighbouring_base_chunk_coefficient = Some(map.next_value()?)
                        }
                        "max_colliding_tiles_coefficient" => {
                            max_colliding_tiles_coefficient = Some(map.next_value()?)
                        }
                        "settler_group_min_size" => {
                            settler_group_min_size = Some(map.next_value()?)
                        }
                        "settler_group_max_size" => {
                            settler_group_max_size = Some(map.next_value()?)
                        }
                        "min_expansion_cooldown" => {
                            min_expansion_cooldown = Some(map.next_value()?)
                        }
                        "max_expansion_cooldown" => {
                            max_expansion_cooldown = Some(map.next_value()?)
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| de::Error::missing_field("enabled"))?;

                Ok(EnemyExpansionSettings {
                    enabled: enabled,
                    max_expansion_distance: max_expansion_distance,
                    friendly_base_influence_radius: friendly_base_influence_radius,
                    enemy_building_influence_radius: enemy_building_influence_radius,
                    building_coefficient: building_coefficient,
                    other_base_coefficient: other_base_coefficient,
                    neighbouring_chunk_coefficient: neighbouring_chunk_coefficient,
                    neighbouring_base_chunk_coefficient: neighbouring_base_chunk_coefficient,
                    max_colliding_tiles_coefficient: max_colliding_tiles_coefficient,
                    settler_group_min_size: settler_group_min_size,
                    settler_group_max_size: settler_group_max_size,
                    min_expansion_cooldown: min_expansion_cooldown,
                    max_expansion_cooldown: max_expansion_cooldown,
                })
            }
        }
        deserializer.deserialize_map(EnemyExpansionSettingsVisitor)
    }
}
