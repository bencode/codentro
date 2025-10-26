# Entrota - Project Context

## Project Overview

Entrota is a code structure analysis tool written in Rust, using Tree-sitter for AST parsing. The project follows a phased development approach with v0.1 focusing on CLI-only functionality.

**Current Version:** v0.1
**License:** MIT
**Design Doc:** `docs/design-v0.1.md`

## Architecture

### Crate Structure

```
/crates
  /entrota-core        # Core engine (AST, IR, graph, metrics) - NO HTTP/UI deps
  /entrota-adapters    # Language adapters (TypeScript/JS/TSX via Tree-sitter)
  /entrota-cli         # CLI interface (view/scan commands)
```

**Key Principle:** `entrota-core` has ZERO dependency on CLI/HTTP/Frontend. This ensures the core can be reused in v0.2 (MCP server) and v0.3 (Web server) without modification.

### Core Types

- `Symbol`: Represents code symbols (class, function, interface, type, enum, const)
- `ModuleIR`: Intermediate representation of a file/module
- `DepEdge`: Dependency edge with source, target, relation, and strength
- `DependencyGraph`: Built on `petgraph` for dependency analysis

## Development Guidelines

### Code Style

- Functional programming style preferred
- Files < 200 lines, functions < 40 lines
- No unnecessary try/catch - let errors propagate naturally
- Type over interface (TypeScript convention carried to Rust philosophy)
- Named exports over default exports

### Testing

- Core logic MUST have unit tests
- Test files: `src/module_name_test.rs` with `#[cfg(test)]` and `#[path = "..."]` pattern
- Current coverage: metrics module (7 tests), graph module (5 tests)

### Tree-sitter Integration

**Important:** Tree-sitter 0.24.x API requires careful iteration:

```rust
// Correct way to use QueryCursor
let mut cursor = QueryCursor::new();
let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());

while let Some((m, _)) = captures.next() {
    // Process captures
}
```

**Language versions:**
- `tree-sitter = "0.24"`
- `tree-sitter-typescript = "0.23"` (0.25 not yet released)
- `tree-sitter-javascript = "0.23"`

### Adapter Implementation

TypeScript adapter uses node walking instead of queries (simplified for v0.1):

```rust
fn walk_node(&self, node: tree_sitter::Node, source: &str, symbols: &mut Vec<Symbol>) {
    match node.kind() {
        "class_declaration" => extract_with_field("name"),
        "function_declaration" => extract_with_field("name"),
        // ...
    }
    for child in node.children() {
        self.walk_node(child, source, symbols);
    }
}
```

## Building & Running

### Build

```bash
cargo build --release
# Binary at: target/release/entrota
```

### Usage Examples

```bash
# View file analysis (table format)
./target/release/entrota view examples/sample.ts

# JSON output
./target/release/entrota view path/to/file.ts --format json

# Help
./target/release/entrota view --help
```

### Output Formats

- `table`: Human-readable table (default)
- `json`: Machine-parsable JSON (matches design doc schema)
- `md`: Markdown format

## Key Metrics

### Complexity Calculation

Complexity score is a normalized metric (0.0 to 1.0), **not Shannon entropy**:

```rust
pub fn calculate_complexity(module: &ModuleIR) -> f64 {
    let symbol_density = (symbol_count / loc).min(1.0);
    let size_complexity = 1.0 - (1.0 / (1.0 + avg_symbol_size / 50.0));
    (symbol_density * 0.4 + size_complexity * 0.6).clamp(0.0, 1.0)
}
```

**Design rationale:**
- Exposed to users as "complexity" to avoid confusion with information-theoretic entropy
- Internal implementation may use information theory concepts in the future
- Combines symbol density and average symbol size using weighted saturation functions

**Interpretation:**
- < 0.3: Simple
- 0.3-0.6: Medium complexity
- \> 0.6: High complexity (consider refactoring)

### LOC Counting

- Code lines only (excludes comments and blank lines)
- Simple line-based scanner (not AST-based for v0.1)
- Handles `//` and `/* */` comments

## Roadmap Context

- **v0.1** (current): CLI-only, TypeScript/JS/TSX support
- **v0.2** (next): MCP server - same core, new transport layer
- **v0.3**: Optional web server + React frontend
- **v0.4+**: Multi-language, call graphs, refactor simulation

## Notes

- Cargo workspace uses `workspace.dependencies` for version management
- All tests pass with `cargo test` (12 tests as of v0.1)
- Example file at `examples/sample.ts` for quick validation
- Git commits should be descriptive and reference design decisions
