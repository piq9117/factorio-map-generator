use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::create("map-settings.json")?;
    let map_settings = MapSettings {
        difficulty_settings: DifficultySettings {
            technology_price_multiplier: 1,
            spoil_time_modifier: 1,
        },
        pollution: PollutionSettings {
            enable: true,
            comment_min_to_diffuse_1: "hello",
            comment_min_to_diffuse_2: "hello",
            diffusion_rate: 0.0,
            min_to_diffuse: 0.1,
            ageing: 0.0,
            expected_max_per_chunk: 0.0,
            min_to_show_per_chunk: 0.0,
            min_pollution_to_damage_trees: 0.0,
            pollution_with_max_forest_damage: 0.0,
            pollution_restored_per_tree_damage: 0.0,
            pollution_per_tree_damage: 0.0,
            max_pollution_to_restore_trees: 0.0,
            enemy_attack_pollution_consumption_modifier: 0.0,
        },
        steering: SteeringSettings {
            default: StateSteeringSettings {
                radius: 0.0,
                separation_factor: 0.0,
                separation_force: 0.0,
                force_unit_fuzzy_goto_behavior: false,
            },
            moving: StateSteeringSettings {
                radius: 0.0,
                separation_factor: 0.0,
                separation_force: 0.0,
                force_unit_fuzzy_goto_behavior: false,
            },
        },
        enemy_evolution: EnemyEvolutionSettings {
            enabled: false,
            time_factor: 0.0,
            destroy_factor: 0.0,
            pollution_factor: 0.0,
        },
        enemy_expansion: EnemyExpansionSettings {
            enabled: false,
            max_expansion_distance: 0,
            friendly_base_influence_radius: 0,
            enemy_building_influence_radius: 0,
            building_coefficient: 0.0,
            other_base_coefficient: 0.0,
            neighbouring_base_chunk_coefficient: 0.0,
            max_colliding_tiles_coefficient: 0.0,
            settler_group_min_size: 0,
            settler_group_max_size: 0,
            min_expansion_cooldown: 0,
            max_expansion_cooldown: 0,
        },
        unit_group: UnitGroupSettings {
            min_group_gathering_time: 0,
            max_group_gathering_time: 0,
            max_wait_time_for_late_members: 0,
            max_group_radius: 0.0,
            min_group_radius: 0.0,
            max_member_speedup_when_behind: 0.0,
            max_member_slowdown_when_ahead: 0.0,
            max_group_slowdown_factor: 0.0,
            max_group_member_fallback_factor: 0.0,
            member_disown_distance: 0.0,
            tick_tolerance_when_members_arrives: 0,
            max_gathering_unit_groups: 0,
            max_unit_group_size: 0,
        },
        path_finder: PathFinderSettings {
            fwd2bwd_ratio: 0,
            goal_pressure_ratio: 0.0,
            use_path_cache: false,
            max_steps_worked_per_tick: 0.0,
            max_work_done_per_tick: 0,
            short_cache_size: 0,
            long_cache_size: 0,
            short_cache_min_cacheable_distance: 0.0,
            short_cache_min_algo_steps_to_cache: 0,
            long_cache_min_cacheable_distance: 0.0,
            cache_max_connect_to_cache_steps_multiplier: 0,
            cache_accept_path_start_distance_ratio: 0.0,
            cache_accept_path_end_distance_ratio: 0.0,
            negative_cache_accept_path_start_distance_ratio: 0.0,
            negative_cache_accept_path_end_distance_ratio: 0.0,
            cache_path_start_distance_rating_multiplier: 0.0,
            cache_path_end_distance_rating_multiplier: 0.0,
            stale_enemy_with_same_destination_collision_penalty: 0.0,
            ignore_moving_enemy_collision_distance: 0.0,
            enemy_with_different_destination_collision_penalty: 0.0,
            general_entity_collision_penalty: 0.0,
            general_entity_subsequent_collision_penalty: 0.0,
            extended_collision_penalty: 0.0,
            max_clients_to_accept_any_new_request: 0,
            max_clients_to_accept_short_new_request: 0,
            direct_distance_to_consider_short_request: 0,
            short_request_max_steps: 0,
            short_request_ratio: 0.0,
            min_steps_to_check_path_find_termination: 0,
            start_to_goal_cost_multiplier_to_terminate_path_find: 0.0,
            overload_levels: vec![0],
            overload_multipliers: vec![0.0],
            negative_path_cache_delay_internal: 0,
        },
        max_failed_behavior_count: 0,
    };

    serde_json::to_writer(file, &map_settings)?;
    Ok(())
}

struct MapSettings<'a> {
    difficulty_settings: DifficultySettings,
    pollution: PollutionSettings<'a>,
    steering: SteeringSettings,
    enemy_evolution: EnemyEvolutionSettings,
    enemy_expansion: EnemyExpansionSettings,
    unit_group: UnitGroupSettings,
    path_finder: PathFinderSettings,
    max_failed_behavior_count: u32,
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
        s.serialize_field("enemy_expension", &self.enemy_expansion)?;
        s.serialize_field("unit_group", &self.unit_group)?;
        s.serialize_field("path_finder", &self.path_finder)?;
        s.serialize_field("max_failed_behavior_count", &self.max_failed_behavior_count)?;
        s.end()
    }
}

struct DifficultySettings {
    technology_price_multiplier: u8,
    spoil_time_modifier: u8,
}

impl Serialize for DifficultySettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DifficultySettings", 2)?;
        s.serialize_field(
            "technology_price_multiplier",
            &self.technology_price_multiplier,
        )?;
        s.serialize_field("spoil_time_modifier", &self.spoil_time_modifier)?;
        s.end()
    }
}

struct PollutionSettings<'a> {
    enable: bool,
    comment_min_to_diffuse_1: &'a str,
    comment_min_to_diffuse_2: &'a str,
    diffusion_rate: f32,
    min_to_diffuse: f32,
    ageing: f32,
    expected_max_per_chunk: f32,
    min_to_show_per_chunk: f32,
    min_pollution_to_damage_trees: f32,
    pollution_with_max_forest_damage: f32,
    pollution_restored_per_tree_damage: f32,
    pollution_per_tree_damage: f32,
    max_pollution_to_restore_trees: f32,
    enemy_attack_pollution_consumption_modifier: f32,
}

impl<'a> Serialize for PollutionSettings<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Polluion", 2)?;
        s.serialize_field("enable", &self.enable)?;
        s.serialize_field("_comment_min_to_diffuse_1", &self.comment_min_to_diffuse_1)?;
        s.serialize_field("_comment_min_to_diffuse_2", &self.comment_min_to_diffuse_2)?;
        s.serialize_field("diffusion_rate", &self.diffusion_rate)?;
        s.serialize_field("min_to_diffuse", &self.min_to_diffuse)?;
        s.serialize_field("ageing", &self.ageing)?;
        s.serialize_field("expected_max_per_chunk", &self.expected_max_per_chunk)?;
        s.serialize_field("min_to_show_per_chunk", &self.min_to_show_per_chunk)?;
        s.serialize_field(
            "min_pollution_to_damage_trees",
            &self.min_pollution_to_damage_trees,
        )?;
        s.serialize_field(
            "pollution_with_max_forest_damage",
            &self.pollution_with_max_forest_damage,
        )?;
        s.serialize_field(
            "pollution_restored_per_tree_damage",
            &self.pollution_restored_per_tree_damage,
        )?;
        s.serialize_field("pollution_per_tree_damage", &self.pollution_per_tree_damage)?;
        s.serialize_field(
            "max_pollution_to_restore_trees",
            &self.max_pollution_to_restore_trees,
        )?;
        s.serialize_field(
            "enemy_attack_pollution_consumption_modifier",
            &self.enemy_attack_pollution_consumption_modifier,
        )?;
        s.end()
    }
}

struct SteeringSettings {
    default: StateSteeringSettings,
    moving: StateSteeringSettings,
}

impl Serialize for SteeringSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SteeringSettings", 2)?;
        s.serialize_field("default", &self.default)?;
        s.serialize_field("moving", &self.moving)?;
        s.end()
    }
}

struct StateSteeringSettings {
    radius: f32,
    separation_factor: f32,
    separation_force: f32,
    force_unit_fuzzy_goto_behavior: bool,
}

impl Serialize for StateSteeringSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StateSteerSettigs", 4)?;
        s.serialize_field("radius", &self.radius)?;
        s.serialize_field("separation_factor", &self.separation_factor)?;
        s.serialize_field("separation_force", &self.separation_force)?;
        s.serialize_field(
            "force_unit_fuzzy_goto_behavior",
            &self.force_unit_fuzzy_goto_behavior,
        )?;
        s.end()
    }
}

struct EnemyEvolutionSettings {
    enabled: bool,
    time_factor: f32,
    destroy_factor: f32,
    pollution_factor: f32,
}

impl Serialize for EnemyEvolutionSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EnemyEvolutionSettings", 4)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field("time_factor", &self.time_factor)?;
        s.serialize_field("destroy_factor", &self.destroy_factor)?;
        s.serialize_field("pollution_factor", &self.pollution_factor)?;
        s.end()
    }
}

struct EnemyExpansionSettings {
    enabled: bool,
    max_expansion_distance: u32,
    friendly_base_influence_radius: u32,
    enemy_building_influence_radius: u32,
    building_coefficient: f32,
    other_base_coefficient: f32,
    neighbouring_base_chunk_coefficient: f32,
    max_colliding_tiles_coefficient: f32,
    settler_group_min_size: u32,
    settler_group_max_size: u32,
    min_expansion_cooldown: u32,
    max_expansion_cooldown: u32,
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

struct UnitGroupSettings {
    min_group_gathering_time: u32,
    max_group_gathering_time: u32,
    max_wait_time_for_late_members: u32,
    max_group_radius: f32,
    min_group_radius: f32,
    max_member_speedup_when_behind: f32,
    max_member_slowdown_when_ahead: f32,
    max_group_slowdown_factor: f32,
    max_group_member_fallback_factor: f32,
    member_disown_distance: f32,
    tick_tolerance_when_members_arrives: u32,
    max_gathering_unit_groups: u32,
    max_unit_group_size: u32,
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

struct PathFinderSettings {
    fwd2bwd_ratio: u32,
    goal_pressure_ratio: f32,
    use_path_cache: bool,
    max_steps_worked_per_tick: f32,
    max_work_done_per_tick: u32,
    short_cache_size: u32,
    long_cache_size: u32,
    short_cache_min_cacheable_distance: f32,
    short_cache_min_algo_steps_to_cache: u32,
    long_cache_min_cacheable_distance: f32,
    cache_max_connect_to_cache_steps_multiplier: u32,
    cache_accept_path_start_distance_ratio: f32,
    cache_accept_path_end_distance_ratio: f32,
    negative_cache_accept_path_start_distance_ratio: f32,
    negative_cache_accept_path_end_distance_ratio: f32,
    cache_path_start_distance_rating_multiplier: f32,
    cache_path_end_distance_rating_multiplier: f32,
    stale_enemy_with_same_destination_collision_penalty: f32,
    ignore_moving_enemy_collision_distance: f32,
    enemy_with_different_destination_collision_penalty: f32,
    general_entity_collision_penalty: f32,
    general_entity_subsequent_collision_penalty: f32,
    extended_collision_penalty: f32,
    max_clients_to_accept_any_new_request: u32,
    max_clients_to_accept_short_new_request: u32,
    direct_distance_to_consider_short_request: u32,
    short_request_max_steps: u32,
    short_request_ratio: f32,
    min_steps_to_check_path_find_termination: u32,
    start_to_goal_cost_multiplier_to_terminate_path_find: f32,
    overload_levels: Vec<u32>,
    overload_multipliers: Vec<f32>,
    negative_path_cache_delay_internal: u32,
}

impl Serialize for PathFinderSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PathFinderSettings", 33)?;
        s.serialize_field("fwd2bwd_ratio", &self.fwd2bwd_ratio)?;
        s.serialize_field("goal_pressure_ratio", &self.goal_pressure_ratio)?;
        s.serialize_field("use_path_cache", &self.use_path_cache)?;
        s.serialize_field("max_steps_worked_per_tick", &self.max_steps_worked_per_tick)?;
        s.serialize_field("max_work_done_per_tick", &self.max_work_done_per_tick)?;
        s.serialize_field("short_cache_size", &self.short_cache_size)?;
        s.serialize_field("long_cache_size", &self.long_cache_size)?;
        s.serialize_field(
            "short_cache_min_cacheable_distance",
            &self.short_cache_min_cacheable_distance,
        )?;
        s.serialize_field(
            "short_cache_min_algo_steps_to_cache",
            &self.short_cache_min_algo_steps_to_cache,
        )?;
        s.serialize_field(
            "long_cache_min_cacheable_distance",
            &self.long_cache_min_cacheable_distance,
        )?;
        s.serialize_field(
            "cache_max_connect_to_cache_steps_multiplier",
            &self.cache_max_connect_to_cache_steps_multiplier,
        )?;
        s.serialize_field(
            "cache_accept_path_start_distance_ratio",
            &self.cache_accept_path_start_distance_ratio,
        )?;
        s.serialize_field(
            "cache_accept_path_end_distance_ratio",
            &self.cache_accept_path_end_distance_ratio,
        )?;
        s.serialize_field(
            "negative_cache_accept_path_start_distance_ratio",
            &self.negative_cache_accept_path_start_distance_ratio,
        )?;
        s.serialize_field(
            "negative_cache_accept_path_end_distance_ratio",
            &self.negative_cache_accept_path_end_distance_ratio,
        )?;
        s.serialize_field(
            "cache_path_start_distance_rating_multiplier",
            &self.cache_path_start_distance_rating_multiplier,
        )?;
        s.serialize_field(
            "cache_path_end_distance_rating_multiplier",
            &self.cache_path_end_distance_rating_multiplier,
        )?;
        s.serialize_field(
            "stale_enemy_with_same_destination_collision_penalty",
            &self.stale_enemy_with_same_destination_collision_penalty,
        )?;
        s.serialize_field(
            "ignore_moving_enemy_collision_distance",
            &self.ignore_moving_enemy_collision_distance,
        )?;
        s.serialize_field(
            "enemy_with_different_destination_collision_penalty",
            &self.enemy_with_different_destination_collision_penalty,
        )?;
        s.serialize_field(
            "general_entity_collision_penalty",
            &self.general_entity_collision_penalty,
        )?;
        s.serialize_field(
            "general_entity_subsequent_collision_penalty",
            &self.general_entity_subsequent_collision_penalty,
        )?;
        s.serialize_field(
            "extended_collision_penalty",
            &self.extended_collision_penalty,
        )?;
        s.serialize_field(
            "max_clients_to_accept_any_new_request",
            &self.max_clients_to_accept_any_new_request,
        )?;
        s.serialize_field(
            "max_clients_to_accept_short_new_request",
            &self.max_clients_to_accept_short_new_request,
        )?;
        s.serialize_field(
            "direct_distance_to_consider_short_request",
            &self.direct_distance_to_consider_short_request,
        )?;
        s.serialize_field("short_request_max_steps", &self.short_request_max_steps)?;
        s.serialize_field("short_request_ratio", &self.short_request_ratio)?;
        s.serialize_field(
            "min_steps_to_check_path_find_termination",
            &self.min_steps_to_check_path_find_termination,
        )?;
        s.serialize_field(
            "start_to_goal_cost_multiplier_to_terminate_path_find",
            &self.start_to_goal_cost_multiplier_to_terminate_path_find,
        )?;
        s.serialize_field("overload_levels", &self.overload_levels)?;
        s.serialize_field("overload_multipliers", &self.overload_multipliers)?;
        s.serialize_field(
            "negative_path_cache_delay_internal",
            &self.negative_path_cache_delay_internal,
        )?;
        s.end()
    }
}
