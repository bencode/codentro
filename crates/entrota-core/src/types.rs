use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
    Class,
    Function,
    Const,
    Variable,
    Interface,
    Type,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
    pub loc: u32,
    /// Complexity score (0.0-1.0), not Shannon entropy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DepKind {
    Import,
    Use,
    Inherit,
    Aggregate,
    Compose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub relation: DepKind,
    pub strength: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleIR {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub loc: u32,
    /// Complexity score (0.0-1.0), aggregated from multiple metrics
    pub complexity: f64,
    #[serde(default)]
    pub symbols: Vec<Symbol>,
    #[serde(default)]
    pub outgoing: Vec<DepEdge>,
    #[serde(default)]
    pub incoming: Vec<DepEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub fan_in: u32,
    pub fan_out: u32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub risk_hotspots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryMetrics {
    pub files: u32,
    pub loc: u32,
    pub avg_complexity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub loc: u32,
    pub complexity: f64,
}

pub type Result<T> = std::result::Result<T, anyhow::Error>;
