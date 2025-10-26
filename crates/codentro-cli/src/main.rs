mod cli;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::View(args) => {
            cli::view::run(args)?;
        }
        Commands::Scan(args) => {
            cli::scan::run(args)?;
        }
    }

    Ok(())
}
