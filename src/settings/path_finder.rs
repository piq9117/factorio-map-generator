use serde::de;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

#[derive(Debug, Clone)]
pub struct PathFinderSettings {
    pub fwd2bwd_ratio: Option<u32>,
    pub goal_pressure_ratio: Option<f32>,
    pub use_path_cache: bool,
    pub max_steps_worked_per_tick: Option<f32>,
    pub max_work_done_per_tick: Option<u32>,
    pub short_cache_size: Option<u32>,
    pub long_cache_size: Option<u32>,
    pub short_cache_min_cacheable_distance: Option<f32>,
    pub short_cache_min_algo_steps_to_cache: Option<u32>,
    pub long_cache_min_cacheable_distance: Option<f32>,
    pub cache_max_connect_to_cache_steps_multiplier: Option<u32>,
    pub cache_accept_path_start_distance_ratio: Option<f32>,
    pub cache_accept_path_end_distance_ratio: Option<f32>,
    pub negative_cache_accept_path_start_distance_ratio: Option<f32>,
    pub negative_cache_accept_path_end_distance_ratio: Option<f32>,
    pub cache_path_start_distance_rating_multiplier: Option<f32>,
    pub cache_path_end_distance_rating_multiplier: Option<f32>,
    pub stale_enemy_with_same_destination_collision_penalty: Option<f32>,
    pub ignore_moving_enemy_collision_distance: Option<f32>,
    pub enemy_with_different_destination_collision_penalty: Option<f32>,
    pub general_entity_collision_penalty: Option<f32>,
    pub general_entity_subsequent_collision_penalty: Option<f32>,
    pub extended_collision_penalty: Option<f32>,
    pub max_clients_to_accept_any_new_request: Option<u32>,
    pub max_clients_to_accept_short_new_request: Option<u32>,
    pub direct_distance_to_consider_short_request: Option<u32>,
    pub short_request_max_steps: Option<u32>,
    pub short_request_ratio: Option<f32>,
    pub min_steps_to_check_path_find_termination: Option<u32>,
    pub start_to_goal_cost_multiplier_to_terminate_path_find: Option<f32>,
    pub overload_levels: Option<Vec<u32>>,
    pub overload_multipliers: Option<Vec<f32>>,
    pub negative_path_cache_delay_interval: Option<u32>,
}

impl Default for PathFinderSettings {
    fn default() -> Self {
        PathFinderSettings {
            fwd2bwd_ratio: Some(0),
            goal_pressure_ratio: Some(0.0),
            use_path_cache: false,
            max_steps_worked_per_tick: Some(0.0),
            max_work_done_per_tick: Some(0),
            short_cache_size: Some(0),
            long_cache_size: Some(0),
            short_cache_min_cacheable_distance: Some(0.0),
            short_cache_min_algo_steps_to_cache: Some(0),
            long_cache_min_cacheable_distance: Some(0.0),
            cache_max_connect_to_cache_steps_multiplier: Some(0),
            cache_accept_path_start_distance_ratio: Some(0.0),
            cache_accept_path_end_distance_ratio: Some(0.0),
            negative_cache_accept_path_start_distance_ratio: Some(0.0),
            negative_cache_accept_path_end_distance_ratio: Some(0.0),
            cache_path_start_distance_rating_multiplier: Some(0.0),
            cache_path_end_distance_rating_multiplier: Some(0.0),
            stale_enemy_with_same_destination_collision_penalty: Some(0.0),
            ignore_moving_enemy_collision_distance: Some(0.0),
            enemy_with_different_destination_collision_penalty: Some(0.0),
            general_entity_collision_penalty: Some(0.0),
            general_entity_subsequent_collision_penalty: Some(0.0),
            extended_collision_penalty: Some(0.0),
            max_clients_to_accept_any_new_request: Some(0),
            max_clients_to_accept_short_new_request: Some(0),
            direct_distance_to_consider_short_request: Some(0),
            short_request_max_steps: Some(0),
            short_request_ratio: Some(0.0),
            min_steps_to_check_path_find_termination: Some(0),
            start_to_goal_cost_multiplier_to_terminate_path_find: Some(0.0),
            overload_levels: Some(vec![0]),
            overload_multipliers: Some(vec![0.0]),
            negative_path_cache_delay_interval: Some(0),
        }
    }
}

impl Serialize for PathFinderSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PathFinderSettings", 33)?;
        s.serialize_field("fwd2bwd_ratio", &self.fwd2bwd_ratio.unwrap_or(2))?;
        s.serialize_field(
            "goal_pressure_ratio",
            &self.goal_pressure_ratio.unwrap_or(0.0),
        )?;
        s.serialize_field("use_path_cache", &self.use_path_cache)?;
        s.serialize_field(
            "max_steps_worked_per_tick",
            &self.max_steps_worked_per_tick.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_work_done_per_tick",
            &self.max_work_done_per_tick.unwrap_or(0),
        )?;
        s.serialize_field("short_cache_size", &self.short_cache_size.unwrap_or(0))?;
        s.serialize_field("long_cache_size", &self.long_cache_size.unwrap_or(0))?;
        s.serialize_field(
            "short_cache_min_cacheable_distance",
            &self.short_cache_min_cacheable_distance.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "short_cache_min_algo_steps_to_cache",
            &self.short_cache_min_algo_steps_to_cache.unwrap_or(0),
        )?;
        s.serialize_field(
            "long_cache_min_cacheable_distance",
            &self.long_cache_min_cacheable_distance.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "cache_max_connect_to_cache_steps_multiplier",
            &self
                .cache_max_connect_to_cache_steps_multiplier
                .unwrap_or(0),
        )?;
        s.serialize_field(
            "cache_accept_path_start_distance_ratio",
            &self.cache_accept_path_start_distance_ratio.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "cache_accept_path_end_distance_ratio",
            &self.cache_accept_path_end_distance_ratio.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "negative_cache_accept_path_start_distance_ratio",
            &self
                .negative_cache_accept_path_start_distance_ratio
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "negative_cache_accept_path_end_distance_ratio",
            &self
                .negative_cache_accept_path_end_distance_ratio
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "cache_path_start_distance_rating_multiplier",
            &self
                .cache_path_start_distance_rating_multiplier
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "cache_path_end_distance_rating_multiplier",
            &self
                .cache_path_end_distance_rating_multiplier
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "stale_enemy_with_same_destination_collision_penalty",
            &self
                .stale_enemy_with_same_destination_collision_penalty
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "ignore_moving_enemy_collision_distance",
            &self.ignore_moving_enemy_collision_distance.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "enemy_with_different_destination_collision_penalty",
            &self
                .enemy_with_different_destination_collision_penalty
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "general_entity_collision_penalty",
            &self.general_entity_collision_penalty.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "general_entity_subsequent_collision_penalty",
            &self
                .general_entity_subsequent_collision_penalty
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "extended_collision_penalty",
            &self.extended_collision_penalty.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "max_clients_to_accept_any_new_request",
            &self.max_clients_to_accept_any_new_request.unwrap_or(0),
        )?;
        s.serialize_field(
            "max_clients_to_accept_short_new_request",
            &self.max_clients_to_accept_short_new_request.unwrap_or(0),
        )?;
        s.serialize_field(
            "direct_distance_to_consider_short_request",
            &self.direct_distance_to_consider_short_request.unwrap_or(0),
        )?;
        s.serialize_field(
            "short_request_max_steps",
            &self.short_request_max_steps.unwrap_or(0),
        )?;
        s.serialize_field(
            "short_request_ratio",
            &self.short_request_ratio.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "min_steps_to_check_path_find_termination",
            &self.min_steps_to_check_path_find_termination.unwrap_or(0),
        )?;
        s.serialize_field(
            "start_to_goal_cost_multiplier_to_terminate_path_find",
            &self
                .start_to_goal_cost_multiplier_to_terminate_path_find
                .unwrap_or(0.0),
        )?;
        s.serialize_field(
            "overload_levels",
            &self.overload_levels.as_ref().unwrap_or(&vec![]),
        )?;
        s.serialize_field(
            "overload_multipliers",
            &self.overload_multipliers.as_ref().unwrap_or(&vec![]),
        )?;
        s.serialize_field(
            "negative_path_cache_delay_interval",
            &self.negative_path_cache_delay_interval.unwrap_or(0),
        )?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for PathFinderSettings {
    fn deserialize<D>(deserializer: D) -> Result<PathFinderSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PathFinderSettingsVisitor;
        impl<'de> Visitor<'de> for PathFinderSettingsVisitor {
            type Value = PathFinderSettings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("PathFinderSettings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut fwd2bwd_ratio: Option<u32> = None;
                let mut goal_pressure_ratio: Option<f32> = None;
                let mut use_path_cache: Option<bool> = None;
                let mut max_steps_worked_per_tick: Option<f32> = None;
                let mut max_work_done_per_tick: Option<u32> = None;
                let mut short_cache_size: Option<u32> = None;
                let mut long_cache_size: Option<u32> = None;
                let mut short_cache_min_cacheable_distance: Option<f32> = None;
                let mut short_cache_min_algo_steps_to_cache: Option<u32> = None;
                let mut long_cache_min_cacheable_distance: Option<f32> = None;
                let mut cache_max_connect_to_cache_steps_multiplier: Option<u32> = None;
                let mut cache_accept_path_start_distance_ratio: Option<f32> = None;
                let mut cache_accept_path_end_distance_ratio: Option<f32> = None;
                let mut negative_cache_accept_path_start_distance_ratio: Option<f32> = None;
                let mut negative_cache_accept_path_end_distance_ratio: Option<f32> = None;
                let mut cache_path_start_distance_rating_multiplier: Option<f32> = None;
                let mut cache_path_end_distance_rating_multiplier: Option<f32> = None;
                let mut stale_enemy_with_same_destination_collision_penalty: Option<f32> = None;
                let mut ignore_moving_enemy_collision_distance: Option<f32> = None;
                let mut enemy_with_different_destination_collision_penalty: Option<f32> = None;
                let mut general_entity_collision_penalty: Option<f32> = None;
                let mut general_entity_subsequent_collision_penalty: Option<f32> = None;
                let mut extended_collision_penalty: Option<f32> = None;
                let mut max_clients_to_accept_any_new_request: Option<u32> = None;
                let mut max_clients_to_accept_short_new_request: Option<u32> = None;
                let mut direct_distance_to_consider_short_request: Option<u32> = None;
                let mut short_request_max_steps: Option<u32> = None;
                let mut short_request_ratio: Option<f32> = None;
                let mut min_steps_to_check_path_find_termination: Option<u32> = None;
                let mut start_to_goal_cost_multiplier_to_terminate_path_find: Option<f32> = None;
                let mut overload_levels: Option<Vec<u32>> = None;
                let mut overload_multipliers: Option<Vec<f32>> = None;
                let mut negative_path_cache_delay_interval: Option<u32> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "fwd2bwd_ratio" => fwd2bwd_ratio = Some(map.next_value()?),
                        "goal_pressure_ratio" => goal_pressure_ratio = Some(map.next_value()?),
                        "use_path_cache" => use_path_cache = Some(map.next_value()?),
                        "max_steps_worked_per_tick" => {
                            max_steps_worked_per_tick = Some(map.next_value()?)
                        }
                        "max_work_done_per_tick" => {
                            max_work_done_per_tick = Some(map.next_value()?)
                        }
                        "short_cache_size" => short_cache_size = Some(map.next_value()?),
                        "long_cache_size" => long_cache_size = Some(map.next_value()?),
                        "short_cache_min_cacheable_distance" => {
                            short_cache_min_cacheable_distance = Some(map.next_value()?)
                        }
                        "short_cache_min_algo_steps_to_cache" => {
                            short_cache_min_algo_steps_to_cache = Some(map.next_value()?)
                        }
                        "long_cache_min_cacheable_distance" => {
                            long_cache_min_cacheable_distance = Some(map.next_value()?)
                        }
                        "cache_max_connect_to_cache_steps_multiplier" => {
                            cache_max_connect_to_cache_steps_multiplier = Some(map.next_value()?)
                        }
                        "cache_accept_path_start_distance_ratio" => {
                            cache_accept_path_start_distance_ratio = Some(map.next_value()?)
                        }
                        "cache_accept_path_end_distance_ratio" => {
                            cache_accept_path_end_distance_ratio = Some(map.next_value()?)
                        }
                        "negative_cache_accept_path_start_distance_ratio" => {
                            negative_cache_accept_path_start_distance_ratio =
                                Some(map.next_value()?)
                        }
                        "negative_cache_accept_path_end_distance_ratio" => {
                            negative_cache_accept_path_end_distance_ratio = Some(map.next_value()?)
                        }
                        "cache_path_start_distance_rating_multiplier" => {
                            cache_path_start_distance_rating_multiplier = Some(map.next_value()?)
                        }
                        "cache_path_end_distance_rating_multiplier" => {
                            cache_path_end_distance_rating_multiplier = Some(map.next_value()?)
                        }
                        "stale_enemy_with_same_destination_collision_penalty" => {
                            stale_enemy_with_same_destination_collision_penalty =
                                Some(map.next_value()?)
                        }
                        "ignore_moving_enemy_collision_distance" => {
                            ignore_moving_enemy_collision_distance = Some(map.next_value()?)
                        }
                        "enemy_with_different_destination_collision_penalty" => {
                            enemy_with_different_destination_collision_penalty =
                                Some(map.next_value()?)
                        }
                        "general_entity_collision_penalty" => {
                            general_entity_collision_penalty = Some(map.next_value()?)
                        }
                        "general_entity_subsequent_collision_penalty" => {
                            general_entity_subsequent_collision_penalty = Some(map.next_value()?)
                        }
                        "extended_collision_penalty" => {
                            extended_collision_penalty = Some(map.next_value()?)
                        }
                        "max_clients_to_accept_any_new_request" => {
                            max_clients_to_accept_any_new_request = Some(map.next_value()?)
                        }
                        "max_clients_to_accept_short_new_request" => {
                            max_clients_to_accept_short_new_request = Some(map.next_value()?)
                        }
                        "direct_distance_to_consider_short_request" => {
                            direct_distance_to_consider_short_request = Some(map.next_value()?)
                        }
                        "short_request_max_steps" => {
                            short_request_max_steps = Some(map.next_value()?)
                        }
                        "short_request_ratio" => short_request_ratio = Some(map.next_value()?),
                        "min_steps_to_check_path_find_termination" => {
                            min_steps_to_check_path_find_termination = Some(map.next_value()?)
                        }
                        "start_to_goal_cost_multiplier_to_terminate_path_find" => {
                            start_to_goal_cost_multiplier_to_terminate_path_find =
                                Some(map.next_value()?)
                        }
                        "overload_levels" => overload_levels = Some(map.next_value()?),
                        "overload_multipliers" => overload_multipliers = Some(map.next_value()?),
                        "negative_path_cache_delay_interval" => {
                            negative_path_cache_delay_interval = Some(map.next_value()?)
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let use_path_cache =
                    use_path_cache.ok_or_else(|| de::Error::missing_field("use_path_cache"))?;

                Ok(PathFinderSettings {
                    fwd2bwd_ratio: fwd2bwd_ratio,
                    goal_pressure_ratio: goal_pressure_ratio,
                    use_path_cache: use_path_cache,
                    max_steps_worked_per_tick: max_steps_worked_per_tick,
                    max_work_done_per_tick: max_work_done_per_tick,
                    short_cache_size: short_cache_size,
                    long_cache_size: long_cache_size,
                    short_cache_min_cacheable_distance: short_cache_min_cacheable_distance,
                    short_cache_min_algo_steps_to_cache: short_cache_min_algo_steps_to_cache,
                    long_cache_min_cacheable_distance: long_cache_min_cacheable_distance,
                    cache_max_connect_to_cache_steps_multiplier:
                        cache_max_connect_to_cache_steps_multiplier,
                    cache_accept_path_start_distance_ratio: cache_accept_path_start_distance_ratio,
                    cache_accept_path_end_distance_ratio: cache_accept_path_end_distance_ratio,
                    negative_cache_accept_path_start_distance_ratio:
                        negative_cache_accept_path_start_distance_ratio,
                    negative_cache_accept_path_end_distance_ratio:
                        negative_cache_accept_path_end_distance_ratio,
                    cache_path_start_distance_rating_multiplier:
                        cache_path_start_distance_rating_multiplier,
                    cache_path_end_distance_rating_multiplier:
                        cache_path_end_distance_rating_multiplier,
                    stale_enemy_with_same_destination_collision_penalty:
                        stale_enemy_with_same_destination_collision_penalty,
                    ignore_moving_enemy_collision_distance: ignore_moving_enemy_collision_distance,
                    enemy_with_different_destination_collision_penalty:
                        enemy_with_different_destination_collision_penalty,
                    general_entity_collision_penalty: general_entity_collision_penalty,
                    general_entity_subsequent_collision_penalty:
                        general_entity_subsequent_collision_penalty,
                    extended_collision_penalty: extended_collision_penalty,
                    max_clients_to_accept_any_new_request: max_clients_to_accept_any_new_request,
                    max_clients_to_accept_short_new_request:
                        max_clients_to_accept_short_new_request,
                    direct_distance_to_consider_short_request:
                        direct_distance_to_consider_short_request,
                    short_request_max_steps: short_request_max_steps,
                    short_request_ratio: short_request_ratio,
                    min_steps_to_check_path_find_termination:
                        min_steps_to_check_path_find_termination,
                    start_to_goal_cost_multiplier_to_terminate_path_find:
                        start_to_goal_cost_multiplier_to_terminate_path_find,
                    overload_levels: overload_levels,
                    overload_multipliers: overload_multipliers,
                    negative_path_cache_delay_interval: negative_path_cache_delay_interval,
                })
            }
        }
        deserializer.deserialize_map(PathFinderSettingsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_path_finder_settings() {
        insta::assert_json_snapshot!(PathFinderSettings::default())
    }
}
