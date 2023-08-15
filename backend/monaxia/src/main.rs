mod cli;
mod misc;
mod repository_impl;
mod web;
mod worker;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
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
