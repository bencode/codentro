# Entrota

Code structure analysis tool for understanding complexity and dependencies.

## Overview

Entrota analyzes code structure using Tree-sitter to extract meaningful insights about:

- **Symbols**: Classes, functions, interfaces, types, and enums
- **Metrics**: Lines of code (LOC) and entropy (complexity measure)
- **Dependencies**: Import relationships and coupling strength
- **Suggestions**: Actionable refactoring recommendations

## Current Status: v0.1 (CLI-only)

This version provides CLI commands for analyzing TypeScript, JavaScript, and TSX files.

## Installation

```bash
cargo install --path crates/entrota-cli
```

Or build from source:

```bash
cargo build --release
```

The binary will be at `target/release/entrota`.

## Usage

### View File Analysis

```bash
entrota view <path> [options]
```

**Options:**
- `--format <table|json|md>` - Output format (default: table)
- `--sort <entropy|loc>` - Sort symbols by entropy or LOC (default: entropy)
- `--depth <N>` - Maximum depth for directory traversal
- `--no-suggest` - Hide refactoring suggestions

**Example:**

```bash
# Analyze a TypeScript file
entrota view src/core/graph.ts

# Get JSON output
entrota view src/core/graph.ts --format json

# Analyze a directory
entrota view src/
```

**Sample output (table format):**

```
Target: src/core/graph.ts (lang=Some("typescript"), LOC=85, Entropy=0.62)

[Structure]
Type         Name                      LOC    Entropy
------------ ------------------------- ------ --------
class        GraphBuilder              45     0.58
function     buildGraph                20     0.65
function     computeEntropy            15     0.71

[Outgoing]
Target                        Relation     Strength
----------------------------- ------------ --------
src/utils/math.ts             import       0.70

[Metrics]
LOC=85 · Entropy=0.62
```

### Scan & Cache

```bash
entrota scan <path> [options]
```

**Options:**
- `--out <file.json>` - Save results to JSON file
- `--format <table|json>` - Output format if --out not specified

**Note:** Scan functionality is planned but not yet implemented in v0.1.

## Architecture

Entrota follows a clean, modular architecture:

```
/crates
  /entrota-core        # Core engine (AST, IR, graph, metrics)
  /entrota-adapters    # Language adapters (TypeScript/JS/TSX)
  /entrota-cli         # CLI interface
```

### Key Concepts

**Entropy**: A normalized complexity measure (0.0 to 1.0) combining:
- Symbol density (symbols per LOC)
- Average symbol size
- Nesting depth (future)
- Coupling metrics (future)

Higher entropy suggests code that may benefit from refactoring.

**Dependency Graph**: Files and their import relationships, enabling:
- Fan-in/fan-out analysis
- Circular dependency detection (future)
- Impact analysis (future)

## Development

### Run Tests

```bash
cargo test
```

### Run Checks

```bash
cargo check
cargo clippy
```

### Try the CLI

```bash
cargo run -p entrota-cli -- view <path>
```

## Roadmap

- **v0.1** (current): CLI-only analysis for TypeScript/JavaScript/TSX
- **v0.2**: MCP server for LLM integration
- **v0.3**: Web server + frontend UI
- **v0.4+**: Multi-language support, call graphs, time-series analysis

## Contributing

Contributions welcome! Please ensure:

1. Code follows the functional programming style
2. Files stay under 200 lines, functions under 40 lines
3. Core logic includes unit tests
4. No unnecessary try/catch blocks

## License

MIT OR Apache-2.0
