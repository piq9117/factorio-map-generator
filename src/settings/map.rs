use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::settings::{
    asteriods::AsteriodSettings, difficulty::DifficultySettings,
    enemy_evolution::EnemyEvolutionSettings, enemy_expansion::EnemyExpansionSettings,
    path_finder::PathFinderSettings, pollution::PollutionSettings, steering::SteeringSettings,
    unit_group::UnitGroupSettings,
};

pub struct MapSettings<'a> {
    pub difficulty_settings: DifficultySettings,
    pub pollution: PollutionSettings<'a>,
    pub steering: SteeringSettings,
    pub enemy_evolution: EnemyEvolutionSettings,
    pub enemy_expansion: EnemyExpansionSettings,
    pub unit_group: UnitGroupSettings,
    pub path_finder: PathFinderSettings,
    pub max_failed_behavior_count: Option<u32>,
    pub asteriods: AsteriodSettings,
}

impl<'a> Serialize for MapSettings<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MapSettings", 2)?;
        s.serialize_field("difficulty_settings", &self.difficulty_settings)?;
        s.serialize_field("pollution", &self.pollution)?;
        s.serialize_field("steering", &self.steering)?;
        s.serialize_field("enemy_evolution", &self.enemy_evolution)?;
        s.serialize_field("enemy_expansion", &self.enemy_expansion)?;
        s.serialize_field("unit_group", &self.unit_group)?;
        s.serialize_field("path_finder", &self.path_finder)?;
        s.serialize_field(
            "max_failed_behavior_count",
            &self.max_failed_behavior_count.unwrap_or(0),
        )?;
        s.serialize_field("asteriods", &self.asteriods)?;
        s.end()
    }
}
