use serde_json;

use std::env;
use std::fs::{read_to_string, File};
use std::path;

use clap::Parser;

mod settings;
use settings::default::gen_settings;
use settings::map::MapSettings;

mod cli;
use cli::Cli;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    get_map_settings_override(&args.map_settings)?;

    let file_with_path = env::current_dir()?.join("map-settings.json");
    let file = File::create(file_with_path)?;
    let map_settings = gen_settings();

    serde_json::to_writer(file, &map_settings)?;

    let json_file = read_to_string("map.json")?;
    let map: MapSettings = serde_json::from_str(&json_file)?;
    println!("{:?}", map);

    Ok(())
}

fn get_map_settings_override(
    map_settings_path: &Option<path::PathBuf>,
) -> Result<Option<MapSettings>, serde_json::Error> {
    match map_settings_path {
        Some(path) => {
            let file_content = read_to_string(path).expect("failed to read file");
            let map_settings: MapSettings = serde_json::from_str(&file_content)?;
            Ok(Some(map_settings))
        }
        None => Ok(None),
    }
}
