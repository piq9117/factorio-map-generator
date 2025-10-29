use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub struct PollutionSettings<'a> {
    pub enabled: bool,
    pub comment_min_to_diffuse_1: Option<&'a str>,
    pub comment_min_to_diffuse_2: Option<&'a str>,
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

impl<'a> Serialize for PollutionSettings<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Polluion", 2)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field(
            "_comment_min_to_diffuse_1",
            &self.comment_min_to_diffuse_1.unwrap_or(""),
        )?;
        s.serialize_field(
            "_comment_min_to_diffuse_2",
            &self.comment_min_to_diffuse_2.unwrap_or(""),
        )?;
        s.serialize_field("diffusion_ratio", &self.diffusion_ratio.unwrap_or(0.0))?;
        s.serialize_field("min_to_diffuse", &self.min_to_diffuse.unwrap_or(0.0))?;
        s.serialize_field("ageing", &self.ageing.unwrap_or(0.0))?;
        s.serialize_field(
            "expected_max_per_chunk",
            &self.expected_max_per_chunk.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "min_to_show_per_chunk",
            &self.min_to_show_per_chunk.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "min_pollution_to_damage_trees",
            &self.min_pollution_to_damage_trees.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "pollution_with_max_forest_damage",
            &self.pollution_with_max_forest_damage.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "pollution_restored_per_tree_damage",
            &self.pollution_restored_per_tree_damage.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "pollution_per_tree_damage",
            &self.pollution_per_tree_damage.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_pollution_to_restore_trees",
            &self.max_pollution_to_restore_trees.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "enemy_attack_pollution_consumption_modifier",
            &self
                .enemy_attack_pollution_consumption_modifier
                .unwrap_or(0.0),
        )?;
        s.end()
    }
}
