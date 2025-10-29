use serde_json;
use std::env;
use std::fs::File;

mod settings;
use settings::default::mk_default_settings;

fn main() -> std::io::Result<()> {
    let file_with_path = env::current_dir()?.join("map-settings.json");
    let file = File::create(file_with_path)?;
    let map_settings = mk_default_settings();

    serde_json::to_writer(file, &map_settings)?;
    Ok(())
}
