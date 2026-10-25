use anyhow::{Context, Result};
use std::{fs::File, io::Read};

use clap::Parser;

use crate::proxy::Config;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = "config.toml".to_string())]
    pub config: String,
}

pub fn parse_config(path: &String) -> Result<Config> {
    let mut config_str: String = "".to_string();
    let mut file = File::open(path).with_context(|| format!("Config file not found: {}", path))?;
    file.read_to_string(&mut config_str)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    Ok(toml::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file: {}", path))?)
}
