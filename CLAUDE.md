# Codentro - Project Context

## Project Overview

Codentro is a code structure analysis tool written in Rust, using Tree-sitter for AST parsing. The project follows a phased development approach with v0.1 focusing on CLI-only functionality.

**Current Version:** v0.1
**License:** MIT
**Design Doc:** `docs/design-v0.1.md`

## Architecture

### Crate Structure

```
/crates
  /codentro-core        # Core engine - NO HTTP/UI deps
  /codentro-adapters    # Language adapters (TypeScript/JS/TSX)
  /codentro-cli         # CLI interface (view/scan commands)
```

**Key Principle:** `codentro-core` has ZERO dependency on CLI/HTTP/Frontend. This ensures the core can be reused in v0.2 (MCP server) and v0.3 (Web server) without modification.

### Multi-Dimensional Quality Metrics System

**Design Philosophy:** Instead of a single complexity score, Codentro provides **independent, measurable quality metrics** similar to ESLint rules. Each metric represents a distinct code quality dimension.

**Core Types:**
- `QualityMetric`: Individual metric with name, value, threshold, severity, and optional message
- `Severity`: Info | Warning | Error
- `ModuleIR`: Contains `metrics: Vec<QualityMetric>` instead of single complexity field
- `Symbol`: Contains `metrics: Vec<QualityMetric>` for symbol-level analysis

**Metric Categories:**

1. **Size Metrics:**
   - `file_loc`: Code lines (excluding comments/blanks)
   - `comment_lines`: Comment line count
   - `blank_lines`: Empty line count

2. **Structure Metrics:**
   - `function_count`: Number of functions
   - `class_count`: Number of classes
   - `type_definition_count`: Types/interfaces/enums
   - `large_function_count`: Functions exceeding threshold

3. **Coupling Metrics:**
   - `fan_out`: Number of dependencies
   - `fan_in`: Number of dependents
   - `import_count`: Import statement count

### Rule System

**Architecture:**
```rust
pub trait QualityRule: Send + Sync {
    fn name(&self) -> &str;
    fn check_module(&self, module: &ModuleIR) -> Vec<QualityMetric>;
    fn check_symbol(&self, symbol: &Symbol) -> Vec<QualityMetric>;
}
```

**Built-in Rules:**
- `FileSizeRule` - File size checks (default: 300 LOC)
- `FunctionSizeRule` - Function size checks (default: 40 LOC)
- `CouplingRule` - Fan-out and import checks (default: 7 deps, 15 imports)
- `StructureStatsRule` - Symbol count checks (default: 20 functions, 30 types per file)

**Configuration via `.codentro.toml`:**
```toml
[rules]
max_file_loc = 300
max_function_loc = 40
max_functions_per_file = 20
max_types_per_file = 30
max_fan_out = 7
max_imports = 15

[rules.severity]
max_file_loc = "Warning"
max_function_loc = "Warning"
max_fan_out = "Warning"
```

Configuration is loaded from `.codentro.toml` in the analyzed file's parent directory, falling back to defaults if not found.

## Development Guidelines

### Code Style

- Functional programming style preferred
- Files < 200 lines, functions < 40 lines
- No unnecessary try/catch - let errors propagate naturally
- Type over interface (TypeScript convention)
- Named exports over default exports

### Testing

- Core logic MUST have unit tests
- Test files: `src/module_name_test.rs` with `#[cfg(test)]` and `#[path = "..."]` pattern

### Tree-sitter Integration

**Important:** Tree-sitter 0.24.x API requires careful iteration:

```rust
// Correct iteration pattern
let mut cursor = QueryCursor::new();
let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());

while let Some((m, _)) = captures.next() {
    // Process captures
}
```

**Language versions:**
- `tree-sitter = "0.24"`
- `tree-sitter-typescript = "0.23"`
- `tree-sitter-javascript = "0.23"`

### Adapter Implementation

TypeScript adapter uses node walking (simplified for v0.1):

```rust
fn walk_node(&self, node: tree_sitter::Node, source: &str, symbols: &mut Vec<Symbol>) {
    match node.kind() {
        "class_declaration" => extract_symbol(SymbolKind::Class),
        "function_declaration" => extract_symbol(SymbolKind::Function),
        "interface_declaration" => extract_symbol(SymbolKind::Interface),
        "type_alias_declaration" => extract_symbol(SymbolKind::Type),
        "enum_declaration" => extract_symbol(SymbolKind::Enum),
        _ => {}
    }
    // Recursively walk children
}
```

**Note:** Symbols are extracted with `metrics: vec![]` - metrics are populated by rule system in CLI layer, not in adapter.

## Building & Running

### Build

```bash
cargo build --release
# Binary at: target/release/codentro
```

### CLI Usage

```bash
# View file analysis (table format with quality metrics)
./target/release/codentro view path/to/file.ts

# JSON output (matches design doc schema)
./target/release/codentro view path/to/file.ts --format json

# Markdown output
./target/release/codentro view path/to/file.ts --format md
```

### Batch Analysis

Analyze multiple files and generate CSV reports:

```bash
# Build first
cargo build --release

# Install bun (if needed)
curl -fsSL https://bun.sh/install | bash

# Run batch analysis
bun scripts/analyze-batch.ts /path/to/target/directory

# Outputs:
# - analysis-results.json (full JSON)
# - analysis-files.csv (file-level summary)
# - analysis-symbols.csv (symbol-level details)
# - analysis-metrics.csv (quality metrics details)
```

**Note:** Analysis output files are git-ignored via `.gitignore` patterns.

## Key Implementation Notes

### LOC Counting

Implemented in `metrics::count_lines()`:
- Returns `LOCStats { code, comment, blank }`
- Handles `//` and `/* */` comments
- Simple line-based scanner (not AST-based)

### Metrics Application Flow

1. **Adapter parses file** → Returns `ModuleIR` with empty `metrics` vectors
2. **CLI loads config** → Creates `RuleRegistry` from `.codentro.toml`
3. **CLI applies rules** → Populates `module.metrics` and `symbol.metrics`
4. **Output formatter** → Displays metrics in requested format

**Important:** Core (`codentro-core`) and adapters (`codentro-adapters`) are **metrics-agnostic**. They only extract structure. The CLI layer (`codentro-cli`) applies quality rules.

### Config Loading

```rust
// Load from .codentro.toml in parent directory, or use defaults
let config_path = args.path.parent().and_then(|p| {
    let config = p.join(".codentro.toml");
    if config.exists() { Some(config) } else { None }
});
let config = Config::load_or_default(config_path.as_deref());
let registry = config.to_rule_registry();
```

## Roadmap Context

- **v0.1** (current): CLI-only, multi-dimensional metrics, configurable rules
- **v0.2** (next): MCP server + cyclomatic complexity analysis
- **v0.3**: Optional web server + React frontend
- **v0.4+**: Multi-language, call graphs, time-series analysis

## Git Workflow

### Commit Message Style

Do **not** include these in commit messages:
- `🤖 Generated with [Claude Code]`
- `Co-Authored-By: Claude <noreply@anthropic.com>`

User's pre-commit hooks may reject commits with these markers. Use `--no-verify` if needed.

### Analysis Output Files

Ignored by git (`.gitignore`):
```
analysis-*.csv
analysis-*.json
```
