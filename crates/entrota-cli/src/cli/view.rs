use crate::cli::{OutputFormat, ViewArgs};
use crate::output;
use anyhow::Result;
use entrota_adapters::typescript::TypeScriptAdapter;
use entrota_adapters::LanguageAdapter;
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

    let adapter = TypeScriptAdapter::new_typescript()?;
    let module = adapter.parse(&args.path, &source)?;

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
