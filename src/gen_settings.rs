use crate::settings::map::MapSettings;

pub fn gen_settings(map_settings: Option<MapSettings>) -> MapSettings {
    match map_settings {
        None => MapSettings {
            difficulty_settings: None,
            pollution: None,
            steering: None,
            enemy_evolution: None,
            enemy_expansion: None,
            unit_group: None,
            path_finder: None,
            max_failed_behavior_count: None,
            asteroids: None,
        },
        Some(map_settings) => map_settings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_gen_settings() {
        let map_settings = gen_settings(None);
        let json = serde_json::to_string(&map_settings).unwrap();
        insta::assert_snapshot!(json);

    }
}
