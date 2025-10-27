use crate::types::{ModuleIR, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FileHash {
    pub path: String,
    pub size: u64,
    pub mtime: SystemTime,
}

impl FileHash {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            mtime: metadata.modified()?,
        })
    }

    pub fn compute_hash(&self) -> String {
        let input = format!("{}:{}:{:?}", self.path, self.size, self.mtime);
        let hash = blake3::hash(input.as_bytes());
        hash.to_hex().to_string()
    }
}

pub struct AnalysisCache {
    cache: HashMap<String, ModuleIR>,
}

impl AnalysisCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, hash: &str) -> Option<&ModuleIR> {
        self.cache.get(hash)
    }

    pub fn insert(&mut self, hash: String, module: ModuleIR) {
        self.cache.insert(hash, module);
    }

    pub fn contains(&self, hash: &str) -> bool {
        self.cache.contains_key(hash)
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for AnalysisCache {
    fn default() -> Self {
        Self::new()
    }
}
