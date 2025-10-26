pub mod view;
pub mod scan;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "entrota")]
#[command(about = "Code structure analysis tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// View structure and dependencies of a file or directory
    View(ViewArgs),
    /// Scan and cache analysis for a path
    Scan(ScanArgs),
}

#[derive(Parser)]
pub struct ViewArgs {
    /// Path to file or directory to analyze
    pub path: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    /// Sort by entropy or LOC
    #[arg(long, value_enum, default_value = "entropy")]
    pub sort: SortBy,

    /// Maximum depth for directory traversal
    #[arg(long)]
    pub depth: Option<usize>,

    /// Hide suggestions
    #[arg(long)]
    pub no_suggest: bool,
}

#[derive(Parser)]
pub struct ScanArgs {
    /// Path to scan
    pub path: PathBuf,

    /// Output file for JSON results
    #[arg(long)]
    pub out: Option<PathBuf>,

    /// Output format (only used if --out is not specified)
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Md,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SortBy {
    Entropy,
    Loc,
}
