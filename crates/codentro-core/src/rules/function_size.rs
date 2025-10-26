use crate::rules::QualityRule;
use crate::types::{ModuleIR, QualityMetric, Severity, Symbol, SymbolKind};

pub struct FunctionSizeRule {
    pub max_loc: u32,
    pub severity: Severity,
}

impl FunctionSizeRule {
    pub fn new(max_loc: u32, severity: Severity) -> Self {
        Self { max_loc, severity }
    }
}

impl Default for FunctionSizeRule {
    fn default() -> Self {
        Self::new(40, Severity::Warning)
    }
}

impl QualityRule for FunctionSizeRule {
    fn name(&self) -> &str {
        "function_size"
    }

    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric> {
        let large_functions: Vec<_> = module
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Function && s.loc > self.max_loc)
            .collect();

        let mut metrics = vec![];

        if !large_functions.is_empty() {
            let details: Vec<String> = large_functions
                .iter()
                .map(|f| format!("{} ({} LOC)", f.name, f.loc))
                .collect();

            metrics.push(QualityMetric {
                name: "large_function_count".to_string(),
                value: large_functions.len() as f64,
                threshold: Some(0.0),
                severity: self.severity.clone(),
                message: Some(format!(
                    "{} functions exceed {} lines: {}",
                    large_functions.len(),
                    self.max_loc,
                    details.join(", ")
                )),
            });
        }

        metrics
    }

    fn check_symbol(&self, symbol: &Symbol) -> Vec<QualityMetric> {
        if symbol.kind == SymbolKind::Function && symbol.loc > self.max_loc {
            vec![QualityMetric {
                name: "function_size".to_string(),
                value: symbol.loc as f64,
                threshold: Some(self.max_loc as f64),
                severity: self.severity.clone(),
                message: Some(format!(
                    "Function has {} lines, exceeds threshold of {}",
                    symbol.loc, self.max_loc
                )),
            }]
        } else {
            vec![]
        }
    }
}
