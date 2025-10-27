mod cli;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{AnalyzeArgs, Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Analyze(args)) => {
            cli::analyze::run(args)?;
        }
        Some(Commands::Init(args)) => {
            cli::init::run(args)?;
        }
        None => {
            // No subcommand provided - default to analyze with global options
            let args = AnalyzeArgs {
                path: cli.path,
                format: Some(cli.format),
                output: cli.output,
                max_depth: cli.max_depth,
                no_suggest: cli.no_suggest,
                sort: Some(cli.sort),
                config: cli.config,
                include_ignored: cli.include_ignored,
            };
            cli::analyze::run(args)?;
        }
    }

    Ok(())
}
