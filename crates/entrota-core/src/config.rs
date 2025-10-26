use crate::rules::*;
use crate::types::Severity;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub rules: RulesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    #[serde(default = "default_max_file_loc")]
    pub max_file_loc: u32,

    #[serde(default = "default_max_function_loc")]
    pub max_function_loc: u32,

    #[serde(default = "default_max_functions_per_file")]
    pub max_functions_per_file: usize,

    #[serde(default = "default_max_types_per_file")]
    pub max_types_per_file: usize,

    #[serde(default = "default_max_fan_out")]
    pub max_fan_out: usize,

    #[serde(default = "default_max_imports")]
    pub max_imports: usize,

    #[serde(default)]
    pub severity: SeverityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityConfig {
    #[serde(default = "default_file_size_severity")]
    pub max_file_loc: String,

    #[serde(default = "default_function_size_severity")]
    pub max_function_loc: String,

    #[serde(default = "default_coupling_severity")]
    pub max_fan_out: String,
}

fn default_max_file_loc() -> u32 {
    300
}
fn default_max_function_loc() -> u32 {
    40
}
fn default_max_functions_per_file() -> usize {
    20
}
fn default_max_types_per_file() -> usize {
    30
}
fn default_max_fan_out() -> usize {
    7
}
fn default_max_imports() -> usize {
    15
}
fn default_file_size_severity() -> String {
    "Warning".to_string()
}
fn default_function_size_severity() -> String {
    "Warning".to_string()
}
fn default_coupling_severity() -> String {
    "Warning".to_string()
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            max_file_loc: default_max_file_loc(),
            max_function_loc: default_max_function_loc(),
            max_functions_per_file: default_max_functions_per_file(),
            max_types_per_file: default_max_types_per_file(),
            max_fan_out: default_max_fan_out(),
            max_imports: default_max_imports(),
            severity: SeverityConfig::default(),
        }
    }
}

impl Default for SeverityConfig {
    fn default() -> Self {
        Self {
            max_file_loc: default_file_size_severity(),
            max_function_loc: default_function_size_severity(),
            max_fan_out: default_coupling_severity(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rules: RulesConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_or_default(path: Option<&Path>) -> Self {
        if let Some(p) = path {
            Self::load(p).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn to_rule_registry(&self) -> RuleRegistry {
        let mut registry = RuleRegistry::new();

        let file_severity = parse_severity(&self.rules.severity.max_file_loc);
        registry.register(Box::new(file_size::FileSizeRule::new(
            self.rules.max_file_loc,
            file_severity,
        )));

        let func_severity = parse_severity(&self.rules.severity.max_function_loc);
        registry.register(Box::new(function_size::FunctionSizeRule::new(
            self.rules.max_function_loc,
            func_severity,
        )));

        let coupling_severity = parse_severity(&self.rules.severity.max_fan_out);
        registry.register(Box::new(coupling::CouplingRule::new(
            self.rules.max_fan_out,
            self.rules.max_imports,
            coupling_severity,
        )));

        registry.register(Box::new(structure_stats::StructureStatsRule::new(
            self.rules.max_functions_per_file,
            self.rules.max_types_per_file,
            Severity::Warning,
        )));

        registry
    }
}

fn parse_severity(s: &str) -> Severity {
    match s.to_lowercase().as_str() {
        "error" => Severity::Error,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.rules.max_file_loc, 300);
        assert_eq!(config.rules.max_function_loc, 40);
    }

    #[test]
    fn test_parse_severity() {
        assert!(matches!(parse_severity("Error"), Severity::Error));
        assert!(matches!(parse_severity("warning"), Severity::Warning));
        assert!(matches!(parse_severity("info"), Severity::Info));
    }
}
