use crate::cli::{AnalyzeArgs, OutputFormat};
use crate::output;
use anyhow::Result;
use codescope_adapters::typescript::TypeScriptAdapter;
use codescope_adapters::LanguageAdapter;
use codescope_core::Config;
use std::fs;
use std::path::PathBuf;

pub fn run(args: AnalyzeArgs) -> Result<()> {
    // Determine the path to analyze (from args or default to current dir)
    let path = args.path.clone().unwrap_or_else(|| PathBuf::from("."));

    if path.is_file() {
        analyze_file(&path, &args)
    } else if path.is_dir() {
        analyze_directory(&path, &args)
    } else {
        anyhow::bail!("Path does not exist: {}", path.display())
    }
}

fn analyze_file(path: &PathBuf, args: &AnalyzeArgs) -> Result<()> {
    let source = fs::read_to_string(path)?;

    // Load configuration
    let config_path = args.config.clone().or_else(|| {
        path.parent().and_then(|p| {
            let config = p.join(".codescope.toml");
            if config.exists() {
                Some(config)
            } else {
                None
            }
        })
    });
    let config = Config::load_or_default(config_path.as_deref());
    let registry = config.to_rule_registry();

    // Parse the file
    let adapter = TypeScriptAdapter::new_typescript()?;
    let mut module = adapter.parse(path, &source)?;

    // Apply quality rules
    let module_metrics = registry.check_module(&module);
    module.metrics = module_metrics;

    // Apply symbol rules
    for symbol in &mut module.symbols {
        let symbol_metrics = registry.check_symbol(symbol);
        symbol.metrics = symbol_metrics;
    }

    // Determine output format
    let format = args.format.clone().unwrap_or(OutputFormat::Table);
    let no_suggest = args.no_suggest;

    // Output
    if let Some(output_path) = &args.output {
        // Write to file
        let content = match format {
            OutputFormat::Json => serde_json::to_string_pretty(&module)?,
            OutputFormat::Table => {
                // For file output, use markdown format as table is console-only
                return Err(anyhow::anyhow!(
                    "Table format is not supported for file output. Use --format json or --format md"
                ));
            }
            OutputFormat::Md => output::format_module_markdown(&module, no_suggest)?,
        };
        fs::write(output_path, content)?;
        println!("Output written to: {}", output_path.display());
    } else {
        // Print to stdout
        match format {
            OutputFormat::Json => {
                output::print_module_json(&module)?;
            }
            OutputFormat::Table => {
                output::print_module_table(&module, no_suggest)?;
            }
            OutputFormat::Md => {
                output::print_module_markdown(&module, no_suggest)?;
            }
        }
    }

    Ok(())
}

fn analyze_directory(_path: &PathBuf, _args: &AnalyzeArgs) -> Result<()> {
    // Placeholder for directory analysis
    println!("Directory analysis not yet implemented");
    Ok(())
}
