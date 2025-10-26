# 🧩 Entrota — Design Specification (Phased)

**Language:** Rust (core & CLI) · TypeScript (only for future front-end)
**Core Principle:** **Core-first.** Everything is implementable and useful from the CLI alone.

---

## Phase Overview

| Phase    | Scope                            | Deliverables                                                                                                              |
| -------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| **v0.1** | **CLI-only** structural analysis | `entrota view`, `entrota scan`, table/JSON outputs, Tree-sitter adapters (TS/JS/TSX), entropy & dependency metrics, cache |
| **v0.2** | **MCP server** for LLM tools     | MCP endpoints wrapping v0.1 core, same schemas, machine-first outputs                                                     |
| **v0.3** | **Server + Web UI** (optional)   | Warp server + React SPA using the same core & schemas                                                                     |

---

## 1) Crate Layout (Monorepo)

```
/crates
  /entrota-core        # Core engine (AST, IR, graph, metrics) — NO HTTP deps
  /entrota-adapters    # Language adapters + .scm queries (TS/JS/TSX)
  /entrota-cli         # CLI (view/scan) — human tables & JSON
  /entrota-mcp         # Phase v0.2 (MCP server) — thin wrapper over core
  /entrota-server      # Phase v0.3 (Warp server) — optional, reuses core
```

> **Strict rule:** `entrota-core` has zero dependency on CLI/HTTP/Frontend.

---

## 2) v0.1 — CLI-Only

### 2.1 Goals

* Provide **immediate structural insight from the terminal**.
* Output **tables** (human) and **JSON** (machine).
* No server, no UI, no watch mode.
* First languages: **TypeScript / JavaScript / TSX** via **Tree-sitter**.

### 2.2 CLI Commands

#### `entrota view <path> [--format table|json|md] [--sort entropy|loc]`

* Input `path` can be a **file** or a **directory**.
* Outputs the **Main Panel** equivalent:

  * **Structure (composition)** + **Dependencies (outgoing/incoming)**
  * **Metrics & suggestions** (subset of Info Panel)
* Default `--format table`.

**File example (table):**

```
Target: src/core/graph.ts (lang=ts, LOC=321, Entropy=0.73)

[Structure]
Type       Name                 LOC   Entropy
---------  -------------------  ----  -------
class      GraphBuilder         200   0.62
function   buildGraph()          48   0.79
function   computeEntropy()       73   0.81

[Outgoing]
Target                 Relation   Strength
---------------------  --------   --------
src/utils/math.ts      import     0.70
src/core/layout.ts     use        0.30

[Incoming]
Source                 Relation   Strength
---------------------  --------   --------
src/api/graph.ts       import     0.40

[Metrics]
LOC=321 · Entropy=0.73 · FanOut=2 · FanIn=1
[Suggestions]
- Split or isolate high-entropy function 'computeEntropy'
```

**Directory example (table):**

```
Target: src/core/ (files=8, avgEntropy=0.61, LOC=4218)

[Children]
Type   Name         LOC   Entropy
-----  -----------  ----  -------
file   graph.ts     321   0.73
file   layout.ts    289   0.61
...

[Outgoing (aggregated)]
Target                Relation  Files  Strength≈
--------------------  --------  -----  ---------
src/utils/math.ts     import    3      0.8
parser/tokenizer.ts   use       1      0.3

[Incoming (aggregated)]
Source                Relation  Files  Strength≈
--------------------  --------  -----  ---------
api/graph.ts          import    1      0.5

[Metrics]
files=8 · LOC=4218 · avgEntropy=0.61
[Suggestions]
- Extract shared utilities with heavy fan-in/fan-out
```

**JSON output schema (file):**

```json
{
  "path": "src/core/graph.ts",
  "type": "file",
  "language": "ts",
  "loc": 321,
  "entropy": 0.73,
  "structure": [
    {"kind":"class","name":"GraphBuilder","loc":200,"entropy":0.62},
    {"kind":"function","name":"buildGraph","loc":48,"entropy":0.79},
    {"kind":"function","name":"computeEntropy","loc":73,"entropy":0.81}
  ],
  "outgoing": [
    {"target":"src/utils/math.ts","relation":"import","strength":0.7},
    {"target":"src/core/layout.ts","relation":"use","strength":0.3}
  ],
  "incoming": [
    {"source":"src/api/graph.ts","relation":"import","strength":0.4}
  ],
  "metrics": {
    "fan_in": 1,
    "fan_out": 2,
    "risk_hotspots": ["computeEntropy"]
  },
  "suggestions": [
    "Split or isolate high-entropy function 'computeEntropy'"
  ]
}
```

**JSON output schema (directory):**

```json
{
  "path": "src/core/",
  "type": "directory",
  "children": [
    {"name":"graph.ts","type":"file","loc":321,"entropy":0.73},
    {"name":"layout.ts","type":"file","loc":289,"entropy":0.61}
  ],
  "outgoing": [
    {"target":"src/utils/math.ts","relation":"import","files":3,"strength":0.8}
  ],
  "incoming": [
    {"source":"src/api/graph.ts","relation":"import","files":1,"strength":0.5}
  ],
  "metrics": {"files":8,"loc":4218,"avg_entropy":0.61},
  "suggestions": ["Extract shared utilities with heavy fan-in/fan-out"]
}
```

#### `entrota scan <path> [--out <file.json>]`

* Precompute & cache analysis for CI or batch use.
* If `--out` omitted, prints summary to stdout (table or JSON with `--format`).

### 2.3 Core Engine (entrota-core)

**Responsibilities**

* File discovery (respect ignore rules).
* Language detection by extension.
* Tree-sitter parsing (TS/JS/TSX).
* `.scm` queries to extract:

  * `import` / `export`
  * `class` / `function` (top-level)
* Module IR build, dependency edges, metrics & entropy.

**Key Types (simplified):**

```rust
pub struct Symbol {
  pub kind: SymbolKind, // Class | Function | Const | ...
  pub name: String,
  pub loc: u32,
  pub entropy: f64,
}

pub struct ModuleIR {
  pub path: String,
  pub language: Option<String>,
  pub loc: u32,
  pub entropy: f64,
  pub symbols: Vec<Symbol>,
  pub outgoing: Vec<DepEdge>,   // imports/uses
  pub incoming: Vec<DepEdge>,   // populated during graph build
}

pub struct DepEdge {
  pub source: String,
  pub target: String,
  pub relation: DepKind,  // Import | Use | Inherit | Aggregate | Compose ...
  pub strength: f32,      // heuristic 0..1
}
```

**Graph:**

* `petgraph::Graph` built over repository modules (nodes = files).
* Enables subgraph lookups (for dir/file) and fan-in/out metrics.

### 2.4 Entropy & Metrics (v0.1 heuristic)

* **LOC** (code/comment/blank) — simple scanner.
* **Entropy** = weighted combination of:

  * symbol count, max nesting, average function length, coupling proxy (fan-in/out).
* **Suggestions**: simple rules (e.g., high entropy node → split; high fan-in → consider API boundaries).

### 2.5 Performance & Cache

* Hash by `(path, size, mtime)`; parse only changed files.
* Multi-thread parse (Rayon).
* Ignore defaults: `.git/`, `node_modules/`, `dist/`, `build/`, `.cache/`, `coverage/`, `target/`.
* File whitelist (v0.1): `.ts`, `.tsx`, `.js`.

### 2.6 Testing

* Golden tests with fixture repos.
* Snapshot tests for CLI (`--format json`).
* Bench on medium TS monorepo.

---

## 3) v0.2 — MCP Server (for LLMs)

**Goal:** Provide an **MCP** (Model Context Protocol) tool endpoint so LLMs can query Entrota without spawning the CLI each time.

**Notes**

* Reuse **exact JSON schemas** from `entrota view` (module/dir).
* Expose tools like:

  * `entrota.module_info(path: string) -> Module JSON`
  * `entrota.repo_stats(root?: string) -> Stats JSON`
  * `entrota.search(pattern: string) -> Paths[]` (optional)
* Stateless requests; internal cache shared across calls.
* No UI, no HTTP server required; MCP transport per host runtime.

**Validation**

* Provide a "fake" tool call parity test that diff's MCP output vs `entrota view --format json`.

---

## 4) v0.3 — Optional Server + Frontend

**(Not in v0.1 scope; documented for continuity.)**

* Warp HTTP server using `/api/module`, `/api/graph`, `/api/stats`, `/api/status`.
* React SPA with Tree (left), Structure/Deps tables (center), Info panel (right).
* Reuse the same **core** and **schemas**.
* Later add WebSocket and visualization (Cytoscape).

---

## 5) Tree-sitter Integration (v0.1)

**Languages:**

* TypeScript (`tree-sitter-typescript/typescript`)
* JavaScript (`tree-sitter-javascript`)
* TSX (`tree-sitter-typescript/tsx`)

**Queries (.scm) — minimal v0.1:**

```scheme
;; imports
(import_declaration
  (import_clause)? (string (string_fragment) @import.path))

;; exports (named/default)
(export_clause (export_specifier name: (identifier) @export.name))
(export_statement
  (identifier) @export.name)

;; classes & functions
(class_declaration name: (identifier) @class.name)
(function_declaration name: (identifier) @function.name)
```

**Adapter Contract:**

```rust
pub trait LanguageAdapter {
  fn match_ext(&self) -> &'static [&'static str]; // [".ts",".tsx",".js"]
  fn parse(&self, path: &Path, source: &str) -> Result<ModuleIR>;
}
```

---

## 6) JSON Schemas (Stable)

* **Module (file)** and **Directory** schemas above are **authoritative** and **shared** across:

  * CLI (`view --format json`)
  * MCP (v0.2)
  * Server APIs (v0.3)

Any breaking change to schemas → bump minor version & changelog.

---

## 7) CLI UX & Exit Codes

* `0`: success
* `2`: partial (some files skipped; printed warnings)
* `1`: error (invalid path / parse failure fatal)

Flags:

* `--format table|json|md` (default `table`)
* `--sort entropy|loc` (default `entropy`)
* `--depth <n>` (directory: limit child listing; default unlimited)
* `--no-suggest` (hide suggestions)

---

## 8) Roadmap Snapshot

* **v0.1 (this spec)**: CLI core complete, fast & stable.
* **v0.2**: MCP server parity; CI examples for LLM integration.
* **v0.3**: Optional server + SPA; later WebSocket & graph viz.
* **v0.4+**: Function call graph, multi-language adapters, time-series entropy, refactor simulation.

---

## 9) Summary

* Entrota's value is immediately available **from the terminal** in v0.1.
* The **same core** powers MCP (v0.2) and Server/UI (v0.3) without rework.
* Stable **schemas** ensure interoperability for tooling and LLM workflows.
* Tree-sitter is **central** in v0.1: structure & dependency extraction are real, fast, and language-agnostic.
