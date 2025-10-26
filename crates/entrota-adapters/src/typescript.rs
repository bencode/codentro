use crate::LanguageAdapter;
use entrota_core::{
    metrics::{calculate_complexity, calculate_symbol_complexity, count_lines},
    types::{DepEdge, DepKind, ModuleIR, Result, Symbol, SymbolKind},
};
use std::path::Path;
use tree_sitter::Parser;

pub struct TypeScriptAdapter {
    language: tree_sitter::Language,
}

impl TypeScriptAdapter {
    pub fn new_typescript() -> Result<Self> {
        Ok(Self {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        })
    }

    pub fn new_tsx() -> Result<Self> {
        Ok(Self {
            language: tree_sitter_typescript::LANGUAGE_TSX.into(),
        })
    }

    pub fn new_javascript() -> Result<Self> {
        Ok(Self {
            language: tree_sitter_javascript::LANGUAGE.into(),
        })
    }

    fn extract_symbols(&self, source: &str, tree: &tree_sitter::Tree) -> Result<Vec<Symbol>> {
        let mut symbols = Vec::new();

        // For now, return a simplified version to get compilation working
        // We'll enhance this later with proper tree-sitter queries
        let root = tree.root_node();
        self.walk_node(root, source, &mut symbols);

        Ok(symbols)
    }

    fn walk_node(&self, node: tree_sitter::Node, source: &str, symbols: &mut Vec<Symbol>) {
        let kind_str = node.kind();

        let (kind, name_field) = match kind_str {
            "class_declaration" => (Some(SymbolKind::Class), "name"),
            "function_declaration" => (Some(SymbolKind::Function), "name"),
            "interface_declaration" => (Some(SymbolKind::Interface), "name"),
            "type_alias_declaration" => (Some(SymbolKind::Type), "name"),
            "enum_declaration" => (Some(SymbolKind::Enum), "name"),
            _ => (None, ""),
        };

        if let Some(kind) = kind {
            if let Some(name_node) = node.child_by_field_name(name_field) {
                if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                    let start = node.start_position().row;
                    let end = node.end_position().row;
                    let loc = (end - start + 1) as u32;

                    let symbol = Symbol {
                        kind,
                        name: name.to_string(),
                        loc,
                        complexity: None,
                    };
                    let complexity = calculate_symbol_complexity(&symbol);
                    symbols.push(Symbol {
                        complexity: Some(complexity),
                        ..symbol
                    });
                }
            }
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.walk_node(child, source, symbols);
            }
        }
    }

    fn extract_imports(&self, source: &str, tree: &tree_sitter::Tree) -> Result<Vec<DepEdge>> {
        let mut deps = Vec::new();
        let root = tree.root_node();
        self.walk_imports(root, source, &mut deps);
        Ok(deps)
    }

    fn walk_imports(&self, node: tree_sitter::Node, source: &str, deps: &mut Vec<DepEdge>) {
        if node.kind() == "import_statement" {
            if let Some(source_node) = node.child_by_field_name("source") {
                if let Ok(import_path) = source_node.utf8_text(source.as_bytes()) {
                    // Remove quotes from string
                    let cleaned = import_path.trim_matches(|c| c == '"' || c == '\'');
                    if !cleaned.is_empty() {
                        deps.push(DepEdge {
                            source: None,
                            target: Some(cleaned.to_string()),
                            relation: DepKind::Import,
                            strength: 0.7,
                            files: None,
                        });
                    }
                }
            }
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.walk_imports(child, source, deps);
            }
        }
    }
}

impl LanguageAdapter for TypeScriptAdapter {
    fn match_ext(&self) -> &'static [&'static str] {
        &["ts", "tsx", "js"]
    }

    fn parse(&self, path: &Path, source: &str) -> Result<ModuleIR> {
        let mut parser = Parser::new();
        parser.set_language(&self.language)?;

        let tree = parser
            .parse(source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

        let loc_stats = count_lines(source);
        let symbols = self.extract_symbols(source, &tree)?;
        let outgoing = self.extract_imports(source, &tree)?;

        let mut module = ModuleIR {
            path: path.to_string_lossy().to_string(),
            language: entrota_core::discovery::detect_language(path),
            loc: loc_stats.code,
            complexity: 0.0,
            symbols,
            outgoing,
            incoming: Vec::new(),
        };

        module.complexity = calculate_complexity(&module);

        Ok(module)
    }
}
