# Analysis Scripts

## Batch Analysis Tool

A tool for batch analyzing TypeScript projects to collect real-world data for validating quality metrics.

### Usage

```bash
# 1. Build entrota
cargo build --release

# 2. Install bun (if not already installed)
curl -fsSL https://bun.sh/install | bash

# 3. Run batch analysis
bun scripts/analyze-batch.ts /path/to/target/directory

# Example: analyze material-ui packages
bun scripts/analyze-batch.ts /Users/bencode/work/trantor/material-ui/packages
```

### Output

The script generates four files:

1. **analysis-results.json** - Complete JSON results
   - Detailed analysis of all files
   - Quality metrics
   - Error information
   - Statistics

2. **analysis-files.csv** - File-level summary
   - Columns: File, LOC, Comment, Blank, Symbols, Dependencies, Warnings

3. **analysis-symbols.csv** - Symbol-level details
   - Columns: File, Symbol Type, Symbol Name, LOC, Issues

4. **analysis-metrics.csv** - Quality metrics details
   - Columns: File, Metric Name, Value, Threshold, Severity, Message

### Statistics Report

The terminal displays:
- File statistics (total, failed, warnings)
- Quality metrics summary (average value, violations)
- Top 10 largest files
- Symbol type statistics (average LOC, issues)

### Sample Output

```
🔍 Finding TypeScript files in: /path/to/packages
📊 Found 523 TypeScript files

⏳ Processing: 100.0% (523/523)

✅ Analysis complete!

📈 Statistics:
────────────────────────────────────────────────────────────
  Total files analyzed: 520
  Failed files: 3
  Files with warnings: 45
  Average LOC per file: 145.3
  Average comment lines: 12.5
  Average blank lines: 8.2

📊 Quality Metrics Summary:
  Metric                        Avg Value       Violations
  ───────────────────────────────────────────────────────
  large_function_count          1.2             32
  file_loc                      145.3           15
  function_count                8.5             5
  fan_out                       3.2             2

🔥 Top 10 Largest Files:
  1. ⚠ src/components/DataGrid/DataGrid.tsx
     LOC: 1250, Warnings: 3
  ...

🎯 Symbol Type Statistics:
  Type           Count     Avg LOC     Issues
  ──────────────────────────────────────────
  function       2341      28.5        156
  class          156       125.3       12
  interface      892       8.2         0
  type           1456      5.1         0
  ...
```

### Data Analysis

The generated CSV files can be imported into Excel/Google Sheets for further analysis:
- Plot LOC vs warning count scatter charts
- Analyze metric distributions across different symbol types
- Identify refactoring candidates based on multiple quality dimensions

### Custom entrota Path

If entrota is not at the default location:

```bash
bun scripts/analyze-batch.ts /path/to/packages ./path/to/entrota
```
