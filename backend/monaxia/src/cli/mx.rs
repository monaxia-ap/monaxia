use super::CommonOptions;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub enum MxSubcommand {
    Migrate,
}

pub async fn execute_mx_subcommand(_options: CommonOptions, subcommand: MxSubcommand) -> Result<()> {
    match subcommand {
        MxSubcommand::Migrate => {
            println!("Executing migrations");
        }
    }
    Ok(())
}
