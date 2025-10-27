use crate::types::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct FileDiscovery {
    extensions: Vec<String>,
}

impl FileDiscovery {
    pub fn new(extensions: Vec<String>) -> Self {
        Self { extensions }
    }

    pub fn discover(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let walker = WalkBuilder::new(root)
            .hidden(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build();

        for entry in walker.filter_map(|e| e.ok()) {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                if self.extensions.iter().any(|e| e == &ext_str) {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }
}

impl Default for FileDiscovery {
    fn default() -> Self {
        Self::new(vec!["ts".to_string(), "tsx".to_string(), "js".to_string()])
    }
}

pub fn detect_language(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext {
            "ts" => "typescript",
            "tsx" => "tsx",
            "js" => "javascript",
            _ => "unknown",
        })
        .map(String::from)
}
