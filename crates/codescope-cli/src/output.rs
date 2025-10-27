use anyhow::Result;
use codescope_core::types::{ModuleIR, Severity};
use serde_json;

pub fn print_module_json(module: &ModuleIR) -> Result<()> {
    let json = serde_json::to_string_pretty(module)?;
    println!("{}", json);
    Ok(())
}

pub fn print_module_table(module: &ModuleIR, _no_suggest: bool) -> Result<()> {
    println!(
        "Target: {} ({}, {} LOC, {} comment, {} blank)",
        module.path,
        module.language.as_deref().unwrap_or("unknown"),
        module.loc,
        module.comment_lines,
        module.blank_lines
    );
    println!();

    // Print quality metrics
    if !module.metrics.is_empty() {
        println!("[Quality Metrics]");
        println!(
            "{:<12} {:<25} {:<10} {:<10} {:<6}",
            "Category", "Metric", "Value", "Threshold", "Status"
        );
        println!("{}", "-".repeat(75));

        for metric in &module.metrics {
            let category = metric_category(&metric.name);
            let status_icon = match metric.severity {
                Severity::Error => "✗",
                Severity::Warning => "⚠",
                Severity::Info => "✓",
            };

            let threshold_str = metric
                .threshold
                .map(|t| format!("{:.0}", t))
                .unwrap_or_else(|| "-".to_string());

            println!(
                "{:<12} {:<25} {:<10.0} {:<10} {:<6}",
                category,
                metric.name,
                metric.value,
                threshold_str,
                status_icon
            );

            if let Some(msg) = &metric.message {
                println!("             {}", msg);
            }
        }
        println!();
    }

    // Print structure with issues
    if !module.symbols.is_empty() {
        println!("[Structure]");
        println!("{:<12} {:<30} {:<6} {:<30}", "Type", "Name", "LOC", "Issues");
        println!("{}", "-".repeat(80));

        for symbol in &module.symbols {
            let issues: Vec<String> = symbol
                .metrics
                .iter()
                .filter(|m| matches!(m.severity, Severity::Warning | Severity::Error))
                .map(|m| {
                    let icon = match m.severity {
                        Severity::Error => "✗",
                        Severity::Warning => "⚠",
                        Severity::Info => "",
                    };
                    format!("{} {}", icon, m.name.replace('_', " "))
                })
                .collect();

            let issues_str = if issues.is_empty() {
                String::new()
            } else {
                issues.join(", ")
            };

            println!(
                "{:<12} {:<30} {:<6} {:<30}",
                format!("{:?}", symbol.kind).to_lowercase(),
                symbol.name,
                symbol.loc,
                issues_str
            );
        }
        println!();
    }

    // Print outgoing dependencies
    if !module.outgoing.is_empty() {
        println!("[Outgoing]");
        println!("{:<30} {:<12} {:<8}", "Target", "Relation", "Strength");
        println!("{}", "-".repeat(55));

        for dep in &module.outgoing {
            if let Some(target) = &dep.target {
                println!(
                    "{:<30} {:<12} {:<8.2}",
                    target,
                    format!("{:?}", dep.relation).to_lowercase(),
                    dep.strength
                );
            }
        }
        println!();
    }

    // Print incoming dependencies
    if !module.incoming.is_empty() {
        println!("[Incoming]");
        println!("{:<30} {:<12} {:<8}", "Source", "Relation", "Strength");
        println!("{}", "-".repeat(55));

        for dep in &module.incoming {
            if let Some(source) = &dep.source {
                println!(
                    "{:<30} {:<12} {:<8.2}",
                    source,
                    format!("{:?}", dep.relation).to_lowercase(),
                    dep.strength
                );
            }
        }
        println!();
    }

    Ok(())
}

fn metric_category(name: &str) -> &str {
    match name {
        n if n.contains("file") || n.contains("loc") || n.contains("comment") || n.contains("blank") => "Size",
        n if n.contains("function") || n.contains("class") || n.contains("interface") || n.contains("type") => "Structure",
        n if n.contains("fan") || n.contains("import") || n.contains("coupling") => "Coupling",
        _ => "Other",
    }
}

pub fn format_module_markdown(module: &ModuleIR, _no_suggest: bool) -> Result<String> {
    let mut output = String::new();

    output.push_str(&format!("# {}\n\n", module.path));
    output.push_str(&format!(
        "**Language:** {} | **LOC:** {} | **Comment:** {} | **Blank:** {}\n\n",
        module.language.as_deref().unwrap_or("unknown"),
        module.loc,
        module.comment_lines,
        module.blank_lines
    ));

    if !module.metrics.is_empty() {
        output.push_str("## Quality Metrics\n\n");
        output.push_str("| Category | Metric | Value | Threshold | Status |\n");
        output.push_str("|----------|--------|-------|-----------|--------|\n");

        for metric in &module.metrics {
            let category = metric_category(&metric.name);
            let status_icon = match metric.severity {
                Severity::Error => "✗",
                Severity::Warning => "⚠",
                Severity::Info => "✓",
            };

            let threshold_str = metric
                .threshold
                .map(|t| format!("{:.0}", t))
                .unwrap_or_else(|| "-".to_string());

            output.push_str(&format!(
                "| {} | {} | {:.0} | {} | {} |\n",
                category, metric.name, metric.value, threshold_str, status_icon
            ));
        }
        output.push('\n');
    }

    if !module.symbols.is_empty() {
        output.push_str("## Structure\n\n");
        output.push_str("| Type | Name | LOC | Issues |\n");
        output.push_str("|------|------|-----|--------|\n");

        for symbol in &module.symbols {
            let issues: Vec<String> = symbol
                .metrics
                .iter()
                .filter(|m| matches!(m.severity, Severity::Warning | Severity::Error))
                .map(|m| m.name.replace('_', " "))
                .collect();

            let issues_str = if issues.is_empty() {
                "-".to_string()
            } else {
                issues.join(", ")
            };

            output.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                format!("{:?}", symbol.kind),
                symbol.name,
                symbol.loc,
                issues_str
            ));
        }
        output.push('\n');
    }

    Ok(output)
}

pub fn print_module_markdown(module: &ModuleIR, no_suggest: bool) -> Result<()> {
    let output = format_module_markdown(module, no_suggest)?;
    print!("{}", output);
    Ok(())
}
