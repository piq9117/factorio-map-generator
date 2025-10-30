use serde_json;
use std::env;
use std::fs::{read_to_string, File};

mod settings;
use settings::default::mk_default_settings;
use settings::pollution::PollutionSettings;

fn main() -> std::io::Result<()> {
    let file_with_path = env::current_dir()?.join("map-settings.json");
    let file = File::create(file_with_path)?;
    let map_settings = mk_default_settings();

    serde_json::to_writer(file, &map_settings)?;

    let json_file = read_to_string("pollution.json")?;
    let pollution: PollutionSettings = serde_json::from_str(&json_file)?;
    println!("{:?}", pollution);

    Ok(())
}
