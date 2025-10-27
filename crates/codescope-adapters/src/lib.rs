pub mod typescript;

use codescope_core::types::{ModuleIR, Result};
use std::path::Path;

pub trait LanguageAdapter {
    fn match_ext(&self) -> &'static [&'static str];
    fn parse(&self, path: &Path, source: &str) -> Result<ModuleIR>;
}
