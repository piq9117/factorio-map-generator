use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

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
    pub overload_levels: Vec<u32>,
    pub overload_multipliers: Vec<f32>,
    pub negative_path_cache_delay_internal: Option<u32>,
}

impl Serialize for PathFinderSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PathFinderSettings", 33)?;
        s.serialize_field("fwd2bwd_ratio", &self.fwd2bwd_ratio.unwrap_or(0))?;
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
        s.serialize_field("overload_levels", &self.overload_levels)?;
        s.serialize_field("overload_multipliers", &self.overload_multipliers)?;
        s.serialize_field(
            "negative_path_cache_delay_internal",
            &self.negative_path_cache_delay_internal.unwrap_or(0),
        )?;
        s.end()
    }
}
