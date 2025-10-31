use clap::Parser;
use std;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, required(false))]
    pub map_settings: Option<std::path::PathBuf>,
}
