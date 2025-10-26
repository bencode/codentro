use crate::rules::QualityRule;
use crate::types::{ModuleIR, QualityMetric, Severity, Symbol};

pub struct FileSizeRule {
    pub max_loc: u32,
    pub severity: Severity,
}

impl FileSizeRule {
    pub fn new(max_loc: u32, severity: Severity) -> Self {
        Self { max_loc, severity }
    }
}

impl Default for FileSizeRule {
    fn default() -> Self {
        Self::new(300, Severity::Warning)
    }
}

impl QualityRule for FileSizeRule {
    fn name(&self) -> &str {
        "file_size"
    }

    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric> {
        let mut metrics = vec![QualityMetric {
            name: "file_loc".to_string(),
            value: module.loc as f64,
            threshold: Some(self.max_loc as f64),
            severity: Severity::Info,
            message: None,
        }];

        if module.loc > self.max_loc {
            metrics.push(QualityMetric {
                name: "file_size".to_string(),
                value: module.loc as f64,
                threshold: Some(self.max_loc as f64),
                severity: self.severity.clone(),
                message: Some(format!(
                    "File has {} lines, exceeds threshold of {}",
                    module.loc, self.max_loc
                )),
            });
        }

        metrics
    }

    fn check_symbol(&self, _symbol: &Symbol) -> Vec<QualityMetric> {
        vec![]
    }
}
