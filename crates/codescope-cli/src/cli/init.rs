use crate::cli::InitArgs;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"# Codescope Configuration File
# See https://github.com/your-org/codescope for documentation

[rules]
# File size thresholds
max_file_loc = 300              # Maximum lines of code per file
max_function_loc = 40           # Maximum lines per function

# Structure thresholds
max_functions_per_file = 20     # Maximum number of functions per file
max_types_per_file = 30         # Maximum type definitions per file

# Coupling thresholds
max_fan_out = 7                 # Maximum number of dependencies
max_imports = 15                # Maximum number of import statements

# Severity levels for rule violations
[rules.severity]
max_file_loc = "Warning"        # Options: "Info", "Warning", "Error"
max_function_loc = "Warning"
max_fan_out = "Warning"
"#;

pub fn run(args: InitArgs) -> Result<()> {
    // Determine the directory
    let dir = args.path.unwrap_or_else(|| PathBuf::from("."));

    if !dir.exists() {
        anyhow::bail!("Directory does not exist: {}", dir.display());
    }

    if !dir.is_dir() {
        anyhow::bail!("Path is not a directory: {}", dir.display());
    }

    let config_path = dir.join(".codescope.toml");

    // Check if config already exists
    if config_path.exists() && !args.force {
        anyhow::bail!(
            "Config file already exists: {}\nUse --force to overwrite",
            config_path.display()
        );
    }

    // Write config file
    fs::write(&config_path, DEFAULT_CONFIG)?;

    println!("✓ Created config file: {}", config_path.display());
    println!("\nYou can now customize the thresholds in this file.");

    Ok(())
}
