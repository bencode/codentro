pub mod analyze;
pub mod init;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "codescope")]
#[command(about = "Code structure analysis tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to file or directory to analyze (default: current directory)
    #[arg(value_name = "PATH", global = true)]
    pub path: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, global = true, default_value = "table")]
    pub format: OutputFormat,

    /// Write output to file
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Maximum depth for directory traversal
    #[arg(long, global = true)]
    pub max_depth: Option<usize>,

    /// Hide refactoring suggestions
    #[arg(long, global = true)]
    pub no_suggest: bool,

    /// Sort symbols by field
    #[arg(long, value_enum, global = true, default_value = "loc")]
    pub sort: SortBy,

    /// Custom config file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze code structure (default when no command specified)
    Analyze(AnalyzeArgs),

    /// Generate default .codescope.toml config file
    Init(InitArgs),
}

#[derive(Parser)]
pub struct AnalyzeArgs {
    /// Path to file or directory to analyze
    pub path: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Write output to file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Maximum depth for directory traversal
    #[arg(long)]
    pub max_depth: Option<usize>,

    /// Hide refactoring suggestions
    #[arg(long)]
    pub no_suggest: bool,

    /// Sort symbols by field
    #[arg(long, value_enum)]
    pub sort: Option<SortBy>,

    /// Custom config file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Parser)]
pub struct InitArgs {
    /// Directory to create config file in (default: current directory)
    pub path: Option<PathBuf>,

    /// Overwrite existing config file
    #[arg(short, long)]
    pub force: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Md,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SortBy {
    Loc,
    Name,
    Issues,
}
