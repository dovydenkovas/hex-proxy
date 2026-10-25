use anyhow::Result;

use clap::Parser;

use crate::cli::{Cli, parse_config};

mod cli;
mod proxy;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = parse_config(&cli.config)?;

    tracing_subscriber::fmt::init();
    proxy::mainloop(config).await?;

    Ok(())
}
