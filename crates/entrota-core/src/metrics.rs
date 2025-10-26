use crate::types::{ModuleIR, Symbol};

#[derive(Debug, Default)]
pub struct LOCStats {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
}

impl LOCStats {
    pub fn total(&self) -> u32 {
        self.code + self.comment + self.blank
    }
}

pub fn count_lines(source: &str) -> LOCStats {
    let mut stats = LOCStats::default();
    let mut in_block_comment = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            stats.blank += 1;
            continue;
        }

        if trimmed.starts_with("/*") {
            in_block_comment = true;
        }

        if in_block_comment {
            stats.comment += 1;
            if trimmed.ends_with("*/") {
                in_block_comment = false;
            }
            continue;
        }

        if trimmed.starts_with("//") {
            stats.comment += 1;
        } else {
            stats.code += 1;
        }
    }

    stats
}

/// Calculate module complexity score (0.0-1.0)
/// Combines symbol density and average symbol size using a weighted formula
pub fn calculate_complexity(module: &ModuleIR) -> f64 {
    let symbol_count = module.symbols.len() as f64;
    let loc = module.loc as f64;

    if loc == 0.0 {
        return 0.0;
    }

    let symbol_density = (symbol_count / loc).min(1.0);
    let avg_symbol_size = if symbol_count > 0.0 {
        loc / symbol_count
    } else {
        0.0
    };

    let size_complexity = if avg_symbol_size > 0.0 {
        1.0 - (1.0 / (1.0 + avg_symbol_size / 50.0))
    } else {
        0.0
    };

    (symbol_density * 0.4 + size_complexity * 0.6).clamp(0.0, 1.0)
}

/// Calculate symbol complexity score (0.0-1.0) based on LOC
pub fn calculate_symbol_complexity(symbol: &Symbol) -> f64 {
    let loc = symbol.loc as f64;
    if loc == 0.0 {
        return 0.0;
    }

    (1.0 - (1.0 / (1.0 + loc / 30.0))).clamp(0.0, 1.0)
}

pub fn generate_suggestions(module: &ModuleIR, fan_in: u32, fan_out: u32) -> Vec<String> {
    let mut suggestions = Vec::new();

    if module.loc > 500 {
        suggestions.push("Large file detected - consider splitting into smaller modules".to_string());
    }

    if fan_out > 10 {
        suggestions.push(format!("High fan-out ({}) - consider reducing dependencies", fan_out));
    }

    if fan_in > 10 {
        suggestions.push(format!("High fan-in ({}) - consider extracting shared utilities", fan_in));
    }

    for symbol in &module.symbols {
        if symbol.loc > 80 {
            suggestions.push(format!(
                "Large {} '{}' ({} LOC) - consider splitting",
                format!("{:?}", symbol.kind).to_lowercase(),
                symbol.name,
                symbol.loc
            ));
        }
    }

    suggestions
}

#[cfg(test)]
#[path = "metrics_test.rs"]
mod metrics_test;
