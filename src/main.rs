use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json;
use std::fs::File;

mod settings;
use settings::{
    asteriods::AsteriodSettings, enemy_evolution::EnemyEvolutionSettings,
    enemy_expansion::EnemyExpansionSettings, path_finder::PathFinderSettings,
    unit_group::UnitGroupSettings,
};

fn main() -> std::io::Result<()> {
    let file = File::create("map-settings.json")?;
    let map_settings = MapSettings {
        difficulty_settings: DifficultySettings {
            technology_price_multiplier: None,
            spoil_time_modifier: None,
        },
        pollution: PollutionSettings {
            enable: true,
            comment_min_to_diffuse_1: None,
            comment_min_to_diffuse_2: None,
            diffusion_rate: None,
            min_to_diffuse: None,
            ageing: None,
            expected_max_per_chunk: None,
            min_to_show_per_chunk: None,
            min_pollution_to_damage_trees: None,
            pollution_with_max_forest_damage: None,
            pollution_restored_per_tree_damage: None,
            pollution_per_tree_damage: None,
            max_pollution_to_restore_trees: None,
            enemy_attack_pollution_consumption_modifier: None,
        },
        steering: SteeringSettings {
            default: StateSteeringSettings {
                radius: None,
                separation_factor: None,
                separation_force: None,
                force_unit_fuzzy_goto_behavior: false,
            },
            moving: StateSteeringSettings {
                radius: None,
                separation_factor: None,
                separation_force: None,
                force_unit_fuzzy_goto_behavior: false,
            },
        },
        enemy_evolution: EnemyEvolutionSettings {
            enabled: false,
            time_factor: None,
            destroy_factor: None,
            pollution_factor: None,
        },
        enemy_expansion: EnemyExpansionSettings {
            enabled: false,
            max_expansion_distance: None,
            friendly_base_influence_radius: None,
            enemy_building_influence_radius: None,
            building_coefficient: None,
            other_base_coefficient: None,
            neighbouring_base_chunk_coefficient: None,
            max_colliding_tiles_coefficient: None,
            settler_group_min_size: None,
            settler_group_max_size: None,
            min_expansion_cooldown: None,
            max_expansion_cooldown: None,
        },
        unit_group: UnitGroupSettings {
            min_group_gathering_time: None,
            max_group_gathering_time: None,
            max_wait_time_for_late_members: None,
            max_group_radius: None,
            min_group_radius: None,
            max_member_speedup_when_behind: None,
            max_member_slowdown_when_ahead: None,
            max_group_slowdown_factor: None,
            max_group_member_fallback_factor: None,
            member_disown_distance: None,
            tick_tolerance_when_members_arrives: None,
            max_gathering_unit_groups: None,
            max_unit_group_size: None,
        },
        path_finder: PathFinderSettings {
            fwd2bwd_ratio: None,
            goal_pressure_ratio: None,
            use_path_cache: false,
            max_steps_worked_per_tick: None,
            max_work_done_per_tick: None,
            short_cache_size: None,
            long_cache_size: None,
            short_cache_min_cacheable_distance: None,
            short_cache_min_algo_steps_to_cache: None,
            long_cache_min_cacheable_distance: None,
            cache_max_connect_to_cache_steps_multiplier: None,
            cache_accept_path_start_distance_ratio: None,
            cache_accept_path_end_distance_ratio: None,
            negative_cache_accept_path_start_distance_ratio: None,
            negative_cache_accept_path_end_distance_ratio: None,
            cache_path_start_distance_rating_multiplier: None,
            cache_path_end_distance_rating_multiplier: None,
            stale_enemy_with_same_destination_collision_penalty: None,
            ignore_moving_enemy_collision_distance: None,
            enemy_with_different_destination_collision_penalty: None,
            general_entity_collision_penalty: None,
            general_entity_subsequent_collision_penalty: None,
            extended_collision_penalty: None,
            max_clients_to_accept_any_new_request: None,
            max_clients_to_accept_short_new_request: None,
            direct_distance_to_consider_short_request: None,
            short_request_max_steps: None,
            short_request_ratio: None,
            min_steps_to_check_path_find_termination: None,
            start_to_goal_cost_multiplier_to_terminate_path_find: None,
            overload_levels: vec![0],
            overload_multipliers: vec![0.0],
            negative_path_cache_delay_internal: None,
        },
        max_failed_behavior_count: None,
        asteriods: AsteriodSettings {
            spawning_rate: None,
            max_ray_portals_expanded_per_tick: None,
        },
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
    max_failed_behavior_count: Option<u32>,
    asteriods: AsteriodSettings,
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
        s.serialize_field("max_failed_behavior_count", &self.max_failed_behavior_count.unwrap_or(0))?;
        s.serialize_field("asteriods", &self.asteriods)?;
        s.end()
    }
}

struct DifficultySettings {
    technology_price_multiplier: Option<f32>,
    spoil_time_modifier: Option<f32>,
}

impl Serialize for DifficultySettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DifficultySettings", 2)?;
        s.serialize_field(
            "technology_price_multiplier",
            &self.technology_price_multiplier.unwrap_or(0.0),
        )?;
        s.serialize_field(
            "spoil_time_modifier",
            &self.spoil_time_modifier.unwrap_or(0.0),
        )?;
        s.end()
    }
}

struct PollutionSettings<'a> {
    enable: bool,
    comment_min_to_diffuse_1: Option<&'a str>,
    comment_min_to_diffuse_2: Option<&'a str>,
    diffusion_rate: Option<f32>,
    min_to_diffuse: Option<f32>,
    ageing: Option<f32>,
    expected_max_per_chunk: Option<f32>,
    min_to_show_per_chunk: Option<f32>,
    min_pollution_to_damage_trees: Option<f32>,
    pollution_with_max_forest_damage: Option<f32>,
    pollution_restored_per_tree_damage: Option<f32>,
    pollution_per_tree_damage: Option<f32>,
    max_pollution_to_restore_trees: Option<f32>,
    enemy_attack_pollution_consumption_modifier: Option<f32>,
}

impl<'a> Serialize for PollutionSettings<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Polluion", 2)?;
        s.serialize_field("enable", &self.enable)?;
        s.serialize_field(
            "_comment_min_to_diffuse_1",
            &self.comment_min_to_diffuse_1.unwrap_or(""),
        )?;
        s.serialize_field(
            "_comment_min_to_diffuse_2",
            &self.comment_min_to_diffuse_2.unwrap_or(""),
        )?;
        s.serialize_field("diffusion_rate", &self.diffusion_rate.unwrap_or(0.0))?;
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
    radius: Option<f32>,
    separation_factor: Option<f32>,
    separation_force: Option<f32>,
    force_unit_fuzzy_goto_behavior: bool,
}

impl Serialize for StateSteeringSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StateSteerSettigs", 4)?;
        s.serialize_field("radius", &self.radius.unwrap_or(0.0))?;
        s.serialize_field("separation_factor", &self.separation_factor.unwrap_or(0.0))?;
        s.serialize_field("separation_force", &self.separation_force.unwrap_or(0.0))?;
        s.serialize_field(
            "force_unit_fuzzy_goto_behavior",
            &self.force_unit_fuzzy_goto_behavior,
        )?;
        s.end()
    }
}
