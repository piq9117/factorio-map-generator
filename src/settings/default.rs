use crate::settings::map::MapSettings;

pub fn gen_settings() -> MapSettings<'static> {
    return MapSettings {
        difficulty_settings: None,
        pollution: None,
        steering: None,
        enemy_evolution: None,
        enemy_expansion: None,
        unit_group: None,
        path_finder: None,
        max_failed_behavior_count: None,
        asteroids: None,
    };
}
