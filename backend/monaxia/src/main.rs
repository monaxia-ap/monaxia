mod cli;
mod config;
mod constant;
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

#[cfg(test)]
mod tests {
    use crate::web::state::construct_state_test;

    #[test]
    fn tests_work() {
        construct_state_test();
    }
}
