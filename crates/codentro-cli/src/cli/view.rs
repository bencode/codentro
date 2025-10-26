use crate::cli::{OutputFormat, ViewArgs};
use crate::output;
use anyhow::Result;
use codentro_adapters::typescript::TypeScriptAdapter;
use codentro_adapters::LanguageAdapter;
use codentro_core::Config;
use std::fs;

pub fn run(args: ViewArgs) -> Result<()> {
    if args.path.is_file() {
        analyze_file(&args)
    } else if args.path.is_dir() {
        analyze_directory(&args)
    } else {
        anyhow::bail!("Path does not exist: {}", args.path.display())
    }
}

fn analyze_file(args: &ViewArgs) -> Result<()> {
    let source = fs::read_to_string(&args.path)?;

    // Load configuration
    let config_path = args.path.parent().and_then(|p| {
        let config = p.join(".codentro.toml");
        if config.exists() {
            Some(config)
        } else {
            None
        }
    });
    let config = Config::load_or_default(config_path.as_deref());
    let registry = config.to_rule_registry();

    // Parse the file
    let adapter = TypeScriptAdapter::new_typescript()?;
    let mut module = adapter.parse(&args.path, &source)?;

    // Apply quality rules
    let module_metrics = registry.check_module(&module);
    module.metrics = module_metrics;

    // Apply symbol rules
    for symbol in &mut module.symbols {
        let symbol_metrics = registry.check_symbol(symbol);
        symbol.metrics = symbol_metrics;
    }

    // Output
    match args.format {
        OutputFormat::Json => {
            output::print_module_json(&module)?;
        }
        OutputFormat::Table => {
            output::print_module_table(&module, args.no_suggest)?;
        }
        OutputFormat::Md => {
            output::print_module_markdown(&module, args.no_suggest)?;
        }
    }

    Ok(())
}

fn analyze_directory(_args: &ViewArgs) -> Result<()> {
    // Placeholder for directory analysis
    println!("Directory analysis not yet implemented");
    Ok(())
}
