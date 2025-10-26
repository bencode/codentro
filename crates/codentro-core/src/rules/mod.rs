use crate::types::{ModuleIR, QualityMetric, Symbol};

pub mod file_size;
pub mod function_size;
pub mod coupling;
pub mod structure_stats;

pub trait QualityRule: Send + Sync {
    fn name(&self) -> &str;
    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric>;
    fn check_symbol(&self, symbol: &Symbol) -> Vec<QualityMetric>;
}

pub struct RuleRegistry {
    rules: Vec<Box<dyn QualityRule>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Box<dyn QualityRule>) {
        self.rules.push(rule);
    }

    pub fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric> {
        self.rules
            .iter()
            .flat_map(|rule| rule.check_module(module))
            .collect()
    }

    pub fn check_symbol(&self, symbol: &Symbol) -> Vec<QualityMetric> {
        self.rules
            .iter()
            .flat_map(|rule| rule.check_symbol(symbol))
            .collect()
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
