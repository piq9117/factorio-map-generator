use serde::de;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

#[derive(Debug, Clone)]
pub struct PollutionSettings {
    pub enabled: bool,
    pub comment_min_to_diffuse_1: Option<String>,
    pub comment_min_to_diffuse_2: Option<String>,
    pub diffusion_ratio: Option<f32>,
    pub min_to_diffuse: Option<f32>,
    pub ageing: Option<f32>,
    pub expected_max_per_chunk: Option<f32>,
    pub min_to_show_per_chunk: Option<f32>,
    pub min_pollution_to_damage_trees: Option<f32>,
    pub pollution_with_max_forest_damage: Option<f32>,
    pub pollution_restored_per_tree_damage: Option<f32>,
    pub pollution_per_tree_damage: Option<f32>,
    pub max_pollution_to_restore_trees: Option<f32>,
    pub enemy_attack_pollution_consumption_modifier: Option<f32>,
}

impl Default for PollutionSettings {
    fn default() -> Self {
        PollutionSettings {
            enabled: false,
            comment_min_to_diffuse_1: Some("".to_string()),
            comment_min_to_diffuse_2: Some("".to_string()),
            diffusion_ratio: Some(0.0),
            min_to_diffuse: Some(0.0),
            ageing: Some(0.0),
            expected_max_per_chunk: Some(0.0),
            min_to_show_per_chunk: Some(0.0),
            min_pollution_to_damage_trees: Some(0.0),
            pollution_with_max_forest_damage: Some(0.0),
            pollution_restored_per_tree_damage: Some(0.0),
            pollution_per_tree_damage: Some(0.0),
            max_pollution_to_restore_trees: Some(0.0),
            enemy_attack_pollution_consumption_modifier: Some(0.0),
        }
    }
}

impl Serialize for PollutionSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Pollution", 2)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field(
            "_comment_min_to_diffuse_1",
            &self.comment_min_to_diffuse_1.clone().unwrap_or_default(),
        )?;
        s.serialize_field(
            "_comment_min_to_diffuse_2",
            &self.comment_min_to_diffuse_2.clone().unwrap_or_default(),
        )?;
        s.serialize_field("diffusion_ratio", &self.diffusion_ratio.unwrap_or_default())?;
        s.serialize_field("min_to_diffuse", &self.min_to_diffuse.unwrap_or_default())?;
        s.serialize_field("ageing", &self.ageing.unwrap_or_default())?;
        s.serialize_field(
            "expected_max_per_chunk",
            &self.expected_max_per_chunk.unwrap_or_default(),
        )?;
        s.serialize_field(
            "min_to_show_per_chunk",
            &self.min_to_show_per_chunk.unwrap_or_default(),
        )?;
        s.serialize_field(
            "min_pollution_to_damage_trees",
            &self.min_pollution_to_damage_trees.unwrap_or_default(),
        )?;
        s.serialize_field(
            "pollution_with_max_forest_damage",
            &self.pollution_with_max_forest_damage.unwrap_or_default(),
        )?;
        s.serialize_field(
            "pollution_restored_per_tree_damage",
            &self.pollution_restored_per_tree_damage.unwrap_or_default(),
        )?;
        s.serialize_field(
            "pollution_per_tree_damage",
            &self.pollution_per_tree_damage.unwrap_or_default(),
        )?;
        s.serialize_field(
            "max_pollution_to_restore_trees",
            &self.max_pollution_to_restore_trees.unwrap_or_default(),
        )?;
        s.serialize_field(
            "enemy_attack_pollution_consumption_modifier",
            &self
                .enemy_attack_pollution_consumption_modifier
                .unwrap_or_default(),
        )?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for PollutionSettings {
    fn deserialize<D>(deserializer: D) -> Result<PollutionSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PollutionSettingsVisitor;
        impl<'de> Visitor<'de> for PollutionSettingsVisitor {
            type Value = PollutionSettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("PollutionSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut enabled: Option<bool> = None;
                let mut comment_min_to_diffuse_1: Option<String> = None;
                let mut comment_min_to_diffuse_2: Option<String> = None;
                let mut diffusion_ratio: Option<f32> = None;
                let mut min_to_diffuse: Option<f32> = None;
                let mut ageing: Option<f32> = None;
                let mut expected_max_per_chunk: Option<f32> = None;
                let mut min_to_show_per_chunk: Option<f32> = None;
                let mut min_pollution_to_damage_trees: Option<f32> = None;
                let mut pollution_with_max_forest_damage: Option<f32> = None;
                let mut pollution_restored_per_tree_damage: Option<f32> = None;
                let mut pollution_per_tree_damage: Option<f32> = None;
                let mut max_pollution_to_restore_trees: Option<f32> = None;
                let mut enemy_attack_pollution_consumption_modifier: Option<f32> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "enabled" => enabled = Some(map.next_value()?),
                        "_comment_min_to_diffuse_1" => {
                            comment_min_to_diffuse_1 = Some(map.next_value()?)
                        }
                        "_comment_min_to_diffuse_2" => {
                            comment_min_to_diffuse_2 = Some(map.next_value()?)
                        }
                        "diffusion_ratio" => diffusion_ratio = Some(map.next_value()?),
                        "min_to_diffuse" => min_to_diffuse = Some(map.next_value()?),
                        "ageing" => ageing = Some(map.next_value()?),
                        "expected_max_per_chunk" => {
                            expected_max_per_chunk = Some(map.next_value()?)
                        }
                        "min_to_show_per_chunk" => min_to_show_per_chunk = Some(map.next_value()?),
                        "min_pollution_to_damage_trees" => {
                            min_pollution_to_damage_trees = Some(map.next_value()?)
                        }
                        "pollution_with_max_forest_damage" => {
                            pollution_with_max_forest_damage = Some(map.next_value()?)
                        }
                        "pollution_restored_per_tree_damage" => {
                            pollution_restored_per_tree_damage = Some(map.next_value()?)
                        }
                        "pollution_per_tree_damage" => {
                            pollution_per_tree_damage = Some(map.next_value()?)
                        }
                        "max_pollution_to_restore_trees" => {
                            max_pollution_to_restore_trees = Some(map.next_value()?)
                        }
                        "enemy_attack_pollution_consumption_modifier" => {
                            enemy_attack_pollution_consumption_modifier = Some(map.next_value()?)
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let enabled = enabled.ok_or_else(|| de::Error::missing_field("enabled"))?;

                Ok(PollutionSettings {
                    enabled: enabled,
                    comment_min_to_diffuse_1: comment_min_to_diffuse_1,
                    comment_min_to_diffuse_2: comment_min_to_diffuse_2,
                    diffusion_ratio: diffusion_ratio,
                    min_to_diffuse: min_to_diffuse,
                    ageing: ageing,
                    expected_max_per_chunk: expected_max_per_chunk,
                    min_to_show_per_chunk: min_to_show_per_chunk,
                    min_pollution_to_damage_trees: min_pollution_to_damage_trees,
                    pollution_with_max_forest_damage: pollution_with_max_forest_damage,
                    pollution_restored_per_tree_damage: pollution_restored_per_tree_damage,
                    pollution_per_tree_damage: pollution_per_tree_damage,
                    max_pollution_to_restore_trees: max_pollution_to_restore_trees,
                    enemy_attack_pollution_consumption_modifier:
                        enemy_attack_pollution_consumption_modifier,
                })
            }
        }
        deserializer.deserialize_map(PollutionSettingsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pollution_settings() {
        insta::assert_yaml_snapshot!(PollutionSettings::default());
    }
}
