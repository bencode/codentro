use crate::rules::QualityRule;
use crate::types::{ModuleIR, QualityMetric, Severity, Symbol};

pub struct CouplingRule {
    pub max_fan_out: usize,
    pub max_imports: usize,
    pub severity: Severity,
}

impl CouplingRule {
    pub fn new(max_fan_out: usize, max_imports: usize, severity: Severity) -> Self {
        Self {
            max_fan_out,
            max_imports,
            severity,
        }
    }
}

impl Default for CouplingRule {
    fn default() -> Self {
        Self::new(7, 15, Severity::Warning)
    }
}

impl QualityRule for CouplingRule {
    fn name(&self) -> &str {
        "coupling"
    }

    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric> {
        let mut metrics = vec![];

        let fan_out = module.outgoing.len();
        metrics.push(QualityMetric {
            name: "fan_out".to_string(),
            value: fan_out as f64,
            threshold: Some(self.max_fan_out as f64),
            severity: if fan_out > self.max_fan_out {
                self.severity.clone()
            } else {
                Severity::Info
            },
            message: if fan_out > self.max_fan_out {
                Some(format!(
                    "Module has {} dependencies, exceeds threshold of {}",
                    fan_out, self.max_fan_out
                ))
            } else {
                None
            },
        });

        let fan_in = module.incoming.len();
        metrics.push(QualityMetric {
            name: "fan_in".to_string(),
            value: fan_in as f64,
            threshold: None,
            severity: Severity::Info,
            message: None,
        });

        let import_count = module.outgoing.len();
        metrics.push(QualityMetric {
            name: "import_count".to_string(),
            value: import_count as f64,
            threshold: Some(self.max_imports as f64),
            severity: if import_count > self.max_imports {
                self.severity.clone()
            } else {
                Severity::Info
            },
            message: if import_count > self.max_imports {
                Some(format!(
                    "Module has {} imports, exceeds threshold of {}",
                    import_count, self.max_imports
                ))
            } else {
                None
            },
        });

        metrics
    }

    fn check_symbol(&self, _symbol: &Symbol) -> Vec<QualityMetric> {
        vec![]
    }
}
