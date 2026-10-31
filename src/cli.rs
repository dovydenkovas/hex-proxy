// hex-proxy - a tiny, asynchronous TCP reverse proxy daemon.
// Copyright (C) 2026 Aleksandr Dovydenkov <asd@altlinux.org>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use anyhow::{Context, Result};
use std::{fs::File, io::Read};

use clap::Parser;

use crate::proxy::Config;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    after_help = "Copyright (C) 2026 Aleksandr Dovydenkov <asd@altlinux.org>
Distributed under the GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>."
)]
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
