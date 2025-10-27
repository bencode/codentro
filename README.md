# Codescope

Code structure analysis tool for understanding complexity and dependencies.

## Overview

Codescope analyzes code structure using Tree-sitter to provide multi-dimensional quality metrics:

- **Symbols**: Classes, functions, interfaces, types, and enums
- **Code Size**: LOC, comment lines, and blank lines
- **Structure Metrics**: Function count, class count, type definitions
- **Coupling Metrics**: Fan-in, fan-out, and import count
- **Quality Rules**: Configurable thresholds with severity levels

## Current Status: v0.1 (CLI-only)

This version provides CLI commands for analyzing TypeScript, JavaScript, and TSX files.

## Installation

```bash
cargo install --path crates/codescope-cli
```

Or build from source:

```bash
cargo build --release
```

The binary will be at `target/release/codescope`.

## Usage

### View File Analysis

```bash
codescope view <path> [options]
```

**Options:**
- `--format <table|json|md>` - Output format (default: table)
- `--sort <loc>` - Sort symbols by LOC (default: loc)
- `--depth <N>` - Maximum depth for directory traversal
- `--no-suggest` - Hide refactoring suggestions

**Example:**

```bash
# Analyze a TypeScript file
codescope view src/core/graph.ts

# Get JSON output
codescope view src/core/graph.ts --format json

# Analyze a directory
codescope view src/
```

**Sample output (table format):**

```
Target: src/core/graph.ts (typescript, 85 LOC, 12 comment, 5 blank)

[Quality Metrics]
Category     Metric                    Value      Threshold  Status
---------------------------------------------------------------------------
Size         file_loc                  85         300        ✓
Structure    function_count            2          20         ✓
Coupling     fan_out                   1          7          ✓

[Structure]
Type         Name                      LOC    Issues
--------------------------------------------------------------------------------
class        GraphBuilder              45
function     buildGraph                20
function     computeComplexity         50     ⚠ function size

[Outgoing]
Target                        Relation     Strength
----------------------------- ------------ --------
src/utils/math.ts             import       0.70
```

### Scan & Cache

```bash
codescope scan <path> [options]
```

**Options:**
- `--out <file.json>` - Save results to JSON file
- `--format <table|json>` - Output format if --out not specified

**Note:** Scan functionality is planned but not yet implemented in v0.1.

## Architecture

Codescope follows a clean, modular architecture:

```
/crates
  /codescope-core        # Core engine (AST, IR, graph, metrics)
  /codescope-adapters    # Language adapters (TypeScript/JS/TSX)
  /codescope-cli         # CLI interface
```

### Key Concepts

**Multi-dimensional Quality Metrics**: Instead of a single complexity score, Codescope provides multiple independent metrics:

**Size Metrics**:
- `file_loc`: Code lines (excluding comments and blanks)
- `comment_lines`: Comment line count
- `blank_lines`: Empty line count

**Structure Metrics**:
- `function_count`: Number of functions
- `class_count`: Number of classes
- `type_definition_count`: Number of types/interfaces/enums
- `large_function_count`: Functions exceeding size threshold

**Coupling Metrics**:
- `fan_out`: Number of dependencies (imports)
- `fan_in`: Number of dependents
- `import_count`: Total import statements

**Quality Rules**: Each metric has a configurable threshold and severity level (Info/Warning/Error).

**Dependency Graph**: Files and their import relationships, enabling:
- Fan-in/fan-out analysis
- Circular dependency detection (future)
- Impact analysis (future)

### Configuration

Create a `.codescope.toml` file to customize thresholds:

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
cargo run -p codescope-cli -- view <path>
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

MIT
