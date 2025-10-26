use anyhow::Result;
use entrota_core::types::ModuleIR;
use serde_json;

pub fn print_module_json(module: &ModuleIR) -> Result<()> {
    let json = serde_json::to_string_pretty(module)?;
    println!("{}", json);
    Ok(())
}

pub fn print_module_table(module: &ModuleIR, _no_suggest: bool) -> Result<()> {
    println!("Target: {} (lang={:?}, LOC={}, Entropy={:.2})",
        module.path,
        module.language,
        module.loc,
        module.entropy
    );
    println!();

    if !module.symbols.is_empty() {
        println!("[Structure]");
        println!("{:<12} {:<25} {:<6} {:<8}", "Type", "Name", "LOC", "Entropy");
        println!("{}", "-".repeat(60));

        for symbol in &module.symbols {
            println!("{:<12} {:<25} {:<6} {:<8.2}",
                format!("{:?}", symbol.kind).to_lowercase(),
                symbol.name,
                symbol.loc,
                symbol.entropy.unwrap_or(0.0)
            );
        }
        println!();
    }

    if !module.outgoing.is_empty() {
        println!("[Outgoing]");
        println!("{:<30} {:<12} {:<8}", "Target", "Relation", "Strength");
        println!("{}", "-".repeat(55));

        for dep in &module.outgoing {
            if let Some(target) = &dep.target {
                println!("{:<30} {:<12} {:<8.2}",
                    target,
                    format!("{:?}", dep.relation).to_lowercase(),
                    dep.strength
                );
            }
        }
        println!();
    }

    if !module.incoming.is_empty() {
        println!("[Incoming]");
        println!("{:<30} {:<12} {:<8}", "Source", "Relation", "Strength");
        println!("{}", "-".repeat(55));

        for dep in &module.incoming {
            if let Some(source) = &dep.source {
                println!("{:<30} {:<12} {:<8.2}",
                    source,
                    format!("{:?}", dep.relation).to_lowercase(),
                    dep.strength
                );
            }
        }
        println!();
    }

    println!("[Metrics]");
    println!("LOC={} · Entropy={:.2}", module.loc, module.entropy);

    Ok(())
}

pub fn print_module_markdown(module: &ModuleIR, _no_suggest: bool) -> Result<()> {
    println!("# {}", module.path);
    println!();
    println!("**Language:** {:?} | **LOC:** {} | **Entropy:** {:.2}",
        module.language, module.loc, module.entropy);
    println!();

    if !module.symbols.is_empty() {
        println!("## Structure");
        println!();
        println!("| Type | Name | LOC | Entropy |");
        println!("|------|------|-----|---------|");

        for symbol in &module.symbols {
            println!("| {} | {} | {} | {:.2} |",
                format!("{:?}", symbol.kind),
                symbol.name,
                symbol.loc,
                symbol.entropy.unwrap_or(0.0)
            );
        }
        println!();
    }

    Ok(())
}
