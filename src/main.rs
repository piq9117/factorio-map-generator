use serde_json;
use std::env;
use std::fs::{read_to_string, File};

mod settings;
use settings::default::gen_settings;
use settings::map::MapSettings;

fn main() -> std::io::Result<()> {
    let file_with_path = env::current_dir()?.join("map-settings.json");
    let file = File::create(file_with_path)?;
    let map_settings = gen_settings();

    serde_json::to_writer(file, &map_settings)?;

    let json_file = read_to_string("map.json")?;
    let map: MapSettings = serde_json::from_str(&json_file)?;
    println!("{:?}", map);

    Ok(())
}
