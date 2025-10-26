use crate::rules::QualityRule;
use crate::types::{ModuleIR, QualityMetric, Severity, Symbol, SymbolKind};

pub struct StructureStatsRule {
    pub max_functions_per_file: usize,
    pub max_types_per_file: usize,
    pub severity: Severity,
}

impl StructureStatsRule {
    pub fn new(
        max_functions_per_file: usize,
        max_types_per_file: usize,
        severity: Severity,
    ) -> Self {
        Self {
            max_functions_per_file,
            max_types_per_file,
            severity,
        }
    }
}

impl Default for StructureStatsRule {
    fn default() -> Self {
        Self::new(20, 30, Severity::Warning)
    }
}

impl QualityRule for StructureStatsRule {
    fn name(&self) -> &str {
        "structure_stats"
    }

    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric> {
        let mut metrics = vec![];

        // Count by kind
        let function_count = module
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .count();
        let class_count = module
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Class)
            .count();
        let interface_count = module
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Interface)
            .count();
        let type_count = module
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Type)
            .count();
        let enum_count = module
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Enum)
            .count();

        // Function count
        metrics.push(QualityMetric {
            name: "function_count".to_string(),
            value: function_count as f64,
            threshold: Some(self.max_functions_per_file as f64),
            severity: if function_count > self.max_functions_per_file {
                self.severity.clone()
            } else {
                Severity::Info
            },
            message: if function_count > self.max_functions_per_file {
                Some(format!(
                    "File has {} functions, exceeds threshold of {}",
                    function_count, self.max_functions_per_file
                ))
            } else {
                None
            },
        });

        // Other counts (info only)
        metrics.push(QualityMetric {
            name: "class_count".to_string(),
            value: class_count as f64,
            threshold: None,
            severity: Severity::Info,
            message: None,
        });

        metrics.push(QualityMetric {
            name: "interface_count".to_string(),
            value: interface_count as f64,
            threshold: None,
            severity: Severity::Info,
            message: None,
        });

        // Total type definitions
        let total_types = interface_count + type_count + enum_count;
        metrics.push(QualityMetric {
            name: "type_definition_count".to_string(),
            value: total_types as f64,
            threshold: Some(self.max_types_per_file as f64),
            severity: if total_types > self.max_types_per_file {
                self.severity.clone()
            } else {
                Severity::Info
            },
            message: if total_types > self.max_types_per_file {
                Some(format!(
                    "File has {} type definitions, exceeds threshold of {}",
                    total_types, self.max_types_per_file
                ))
            } else {
                None
            },
        });

        // Comment and blank lines
        if module.comment_lines > 0 {
            metrics.push(QualityMetric {
                name: "comment_lines".to_string(),
                value: module.comment_lines as f64,
                threshold: None,
                severity: Severity::Info,
                message: None,
            });
        }

        if module.blank_lines > 0 {
            metrics.push(QualityMetric {
                name: "blank_lines".to_string(),
                value: module.blank_lines as f64,
                threshold: None,
                severity: Severity::Info,
                message: None,
            });
        }

        metrics
    }

    fn check_symbol(&self, _symbol: &Symbol) -> Vec<QualityMetric> {
        vec![]
    }
}
