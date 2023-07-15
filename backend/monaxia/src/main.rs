mod cli;
mod config;
mod web;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Arguments::parse();
    tracing_subscriber::fmt::init();
    cli::execute_cli(args).await?;
    Ok(())
}
