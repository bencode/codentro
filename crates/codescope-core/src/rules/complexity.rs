use crate::rules::QualityRule;
use crate::types::{ModuleIR, QualityMetric, Severity, Symbol, SymbolKind};

pub struct ComplexityRule {
    pub max_complexity: u32,
    pub severity: Severity,
}

impl ComplexityRule {
    pub fn new(max_complexity: u32, severity: Severity) -> Self {
        Self {
            max_complexity,
            severity,
        }
    }
}

impl Default for ComplexityRule {
    fn default() -> Self {
        Self::new(10, Severity::Warning)
    }
}

impl QualityRule for ComplexityRule {
    fn name(&self) -> &str {
        "complexity"
    }

    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric> {
        let complex_functions: Vec<_> = module
            .symbols
            .iter()
            .filter(|s| {
                s.kind == SymbolKind::Function
                    && s.cyclomatic_complexity.unwrap_or(0) > self.max_complexity
            })
            .collect();

        let mut metrics = vec![];

        if !complex_functions.is_empty() {
            let details: Vec<String> = complex_functions
                .iter()
                .map(|f| {
                    format!(
                        "{} (complexity: {})",
                        f.name,
                        f.cyclomatic_complexity.unwrap_or(0)
                    )
                })
                .collect();

            metrics.push(QualityMetric {
                name: "high_complexity_count".to_string(),
                value: complex_functions.len() as f64,
                threshold: Some(0.0),
                severity: self.severity.clone(),
                message: Some(format!(
                    "{} functions exceed complexity threshold of {}: {}",
                    complex_functions.len(),
                    self.max_complexity,
                    details.join(", ")
                )),
            });
        }

        metrics
    }

    fn check_symbol(&self, symbol: &Symbol) -> Vec<QualityMetric> {
        if symbol.kind == SymbolKind::Function {
            if let Some(complexity) = symbol.cyclomatic_complexity {
                if complexity > self.max_complexity {
                    return vec![QualityMetric {
                        name: "cyclomatic_complexity".to_string(),
                        value: complexity as f64,
                        threshold: Some(self.max_complexity as f64),
                        severity: self.severity.clone(),
                        message: Some(format!(
                            "Cyclomatic complexity {} exceeds threshold of {}",
                            complexity, self.max_complexity
                        )),
                    }];
                }
            }
        }
        vec![]
    }
}
