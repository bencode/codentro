use crate::cli::{AnalyzeArgs, OutputFormat};
use crate::output;
use anyhow::Result;
use codescope_adapters::typescript::TypeScriptAdapter;
use codescope_adapters::LanguageAdapter;
use codescope_core::types::{ModuleIR, Severity};
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

fn analyze_directory(path: &PathBuf, args: &AnalyzeArgs) -> Result<()> {
    use std::fs;
    use walkdir::WalkDir;

    // Collect TypeScript/JavaScript files
    let max_depth = args.max_depth.unwrap_or(usize::MAX);
    let mut ts_files = Vec::new();

    for entry in WalkDir::new(path)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                if matches!(ext_str.as_ref(), "ts" | "tsx" | "js" | "jsx") && !path.to_string_lossy().ends_with(".d.ts") {
                    ts_files.push(path.to_path_buf());
                }
            }
        }
    }

    if ts_files.is_empty() {
        println!("No TypeScript/JavaScript files found in {}", path.display());
        return Ok(());
    }

    println!("Found {} TypeScript/JavaScript files", ts_files.len());
    println!();

    // Load configuration
    let config_path = args.config.clone().or_else(|| {
        let config = path.join(".codescope.toml");
        if config.exists() {
            Some(config)
        } else {
            None
        }
    });
    let config = Config::load_or_default(config_path.as_deref());
    let registry = config.to_rule_registry();

    // Analyze each file
    let adapter = TypeScriptAdapter::new_typescript()?;
    let mut all_modules = Vec::new();
    let mut error_count = 0;

    for (i, file_path) in ts_files.iter().enumerate() {
        if (i + 1) % 10 == 0 || i + 1 == ts_files.len() {
            eprint!("\rAnalyzing... {}/{}", i + 1, ts_files.len());
        }

        match fs::read_to_string(file_path) {
            Ok(source) => {
                match adapter.parse(file_path, &source) {
                    Ok(mut module) => {
                        // Apply quality rules
                        let module_metrics = registry.check_module(&module);
                        module.metrics = module_metrics;

                        // Apply symbol rules
                        for symbol in &mut module.symbols {
                            let symbol_metrics = registry.check_symbol(symbol);
                            symbol.metrics = symbol_metrics;
                        }

                        all_modules.push(module);
                    }
                    Err(_) => {
                        error_count += 1;
                    }
                }
            }
            Err(_) => {
                error_count += 1;
            }
        }
    }

    eprintln!("\r✓ Analyzed {} files ({} errors)", all_modules.len(), error_count);
    println!();

    // Aggregate statistics
    print_directory_summary(&all_modules, args)?;

    Ok(())
}

fn print_directory_summary(modules: &[ModuleIR], args: &AnalyzeArgs) -> Result<()> {
    let total_loc: u32 = modules.iter().map(|m| m.loc).sum();
    let total_files = modules.len();
    let avg_loc = if total_files > 0 { total_loc / total_files as u32 } else { 0 };

    // Count files with issues
    let files_with_issues: Vec<_> = modules
        .iter()
        .filter(|m| {
            m.metrics.iter().any(|metric| {
                matches!(metric.severity, Severity::Warning | Severity::Error)
            })
        })
        .collect();

    println!("📊 Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Total files:        {}", total_files);
    println!("  Total LOC:          {}", total_loc);
    println!("  Average LOC/file:   {}", avg_loc);
    println!("  Files with issues:  {} ({:.1}%)",
        files_with_issues.len(),
        if total_files > 0 { files_with_issues.len() as f64 / total_files as f64 * 100.0 } else { 0.0 }
    );
    println!();

    if !files_with_issues.is_empty() {
        println!("⚠  Files with Quality Issues");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let mut sorted_files = files_with_issues.clone();
        sorted_files.sort_by(|a, b| {
            let a_issues = a.metrics.iter().filter(|m| matches!(m.severity, Severity::Warning | Severity::Error)).count();
            let b_issues = b.metrics.iter().filter(|m| matches!(m.severity, Severity::Warning | Severity::Error)).count();
            b_issues.cmp(&a_issues)
        });

        let display_limit = 20;
        for (i, module) in sorted_files.iter().take(display_limit).enumerate() {
            let issues: Vec<_> = module
                .metrics
                .iter()
                .filter(|m| matches!(m.severity, Severity::Warning | Severity::Error))
                .collect();

            println!("{}. {} (LOC: {})", i + 1, module.path, module.loc);
            for issue in issues {
                let icon = match issue.severity {
                    Severity::Error => "✗",
                    Severity::Warning => "⚠",
                    Severity::Info => "ℹ",
                };
                println!("   {} {}", icon, issue.message.as_deref().unwrap_or(&issue.name));
            }
        }

        if sorted_files.len() > display_limit {
            println!();
            println!("... and {} more files with issues", sorted_files.len() - display_limit);
        }
    }

    // Output to file if requested
    if let Some(output_path) = &args.output {
        let format = args.format.clone().unwrap_or(OutputFormat::Json);
        match format {
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(&modules)?;
                fs::write(output_path, json)?;
                println!("\n💾 Full results saved to: {}", output_path.display());
            }
            OutputFormat::Md => {
                return Err(anyhow::anyhow!("Markdown format not yet supported for directory analysis"));
            }
            OutputFormat::Table => {
                return Err(anyhow::anyhow!("Table format is not supported for file output"));
            }
        }
    }

    Ok(())
}
