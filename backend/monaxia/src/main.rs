mod cli;
mod config;
mod constant;
mod data;
mod db;
mod repository;
mod web;

use crate::constant::{SOFTWARE_NAME, VERSION_TAG};

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{} {}", SOFTWARE_NAME, VERSION_TAG);
    let args = cli::Arguments::parse();
    tracing_subscriber::fmt::init();
    cli::execute_cli(args).await?;
    Ok(())
}
