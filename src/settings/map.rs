use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

use crate::settings::{
    asteroids::AsteroidSettings, difficulty::DifficultySettings,
    enemy_evolution::EnemyEvolutionSettings, enemy_expansion::EnemyExpansionSettings,
    path_finder::PathFinderSettings, pollution::PollutionSettings, steering::SteeringSettings,
    unit_group::UnitGroupSettings,
};

#[derive(Debug)]
pub struct MapSettings<'a> {
    pub difficulty_settings: Option<DifficultySettings>,
    pub pollution: Option<PollutionSettings<'a>>,
    pub steering: Option<SteeringSettings>,
    pub enemy_evolution: Option<EnemyEvolutionSettings>,
    pub enemy_expansion: Option<EnemyExpansionSettings>,
    pub unit_group: Option<UnitGroupSettings>,
    pub path_finder: Option<PathFinderSettings>,
    pub max_failed_behavior_count: Option<u32>,
    pub asteroids: Option<AsteroidSettings>,
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
        s.serialize_field("asteroids", &self.asteroids)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for MapSettings<'de> {
    fn deserialize<D>(deserializer: D) -> Result<MapSettings<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapSettingsVisitor;
        impl<'de> Visitor<'de> for MapSettingsVisitor {
            type Value = MapSettings<'de>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("MapSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut difficulty_settings: Option<DifficultySettings> = None;
                let mut pollution: Option<PollutionSettings<'de>> = None;
                let mut steering: Option<SteeringSettings> = None;
                let mut enemy_evolution: Option<EnemyEvolutionSettings> = None;
                let mut enemy_expansion: Option<EnemyExpansionSettings> = None;
                let mut unit_group: Option<UnitGroupSettings> = None;
                let mut path_finder: Option<PathFinderSettings> = None;
                let mut max_failed_behavior_count: Option<u32> = None;
                let mut asteroids: Option<AsteroidSettings> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "difficulty_settings" => difficulty_settings = map.next_value()?,
                        "pollution" => pollution = map.next_value()?,
                        "steering" => steering = map.next_value()?,
                        "enemy_evolution" => enemy_evolution = map.next_value()?,
                        "enemy_expansion" => enemy_expansion = map.next_value()?,
                        "unit_group" => unit_group = map.next_value()?,
                        "path_finder" => path_finder = map.next_value()?,
                        "max_failed_behavior_count" => {
                            max_failed_behavior_count = map.next_value()?
                        }
                        "asteroids" => asteroids = map.next_value()?,
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(MapSettings {
                    difficulty_settings: difficulty_settings,
                    pollution: pollution,
                    steering: steering,
                    enemy_evolution: enemy_evolution,
                    enemy_expansion: enemy_expansion,
                    unit_group: unit_group,
                    path_finder: path_finder,
                    max_failed_behavior_count: max_failed_behavior_count,
                    asteroids: asteroids,
                })
            }
        }
        deserializer.deserialize_map(MapSettingsVisitor)
    }
}
