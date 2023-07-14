use super::CommonOptions;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub enum MxSubcommand {
    /// Execute DB migration.
    Migrate,

    /// Create new migration file.
    #[clap(name = "migrate:new")]
    MigrateNew,
}

pub async fn execute_mx_subcommand(
    _options: CommonOptions,
    subcommand: MxSubcommand,
) -> Result<()> {
    match subcommand {
        MxSubcommand::Migrate => {
            println!("Executing migrations");
        }
        MxSubcommand::MigrateNew => {
            println!("Creating new migration file");
        }
    }
    Ok(())
}
