use serde_json;
use std::fs::File;

mod settings;
use settings::default::mk_default_settings;

fn main() -> std::io::Result<()> {
    let file = File::create("map-settings.json")?;
    let map_settings = mk_default_settings();

    serde_json::to_writer(file, &map_settings)?;
    Ok(())
}
