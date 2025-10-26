# Analysis Scripts

## Batch Analysis Tool

批量分析 TypeScript 项目的工具，用于收集真实数据验证复杂度算法。

### 使用方法

```bash
# 1. 确保已构建 entrota
cargo build --release

# 2. 安装 bun (如果还没有)
curl -fsSL https://bun.sh/install | bash

# 3. 运行批量分析
bun scripts/analyze-batch.ts /path/to/target/directory

# 例如：分析 material-ui packages
bun scripts/analyze-batch.ts /Users/bencode/work/trantor/material-ui/packages
```

### 输出

脚本会生成三个文件：

1. **analysis-results.json** - 完整的 JSON 结果
   - 包含所有文件的详细分析
   - 错误信息
   - 统计数据

2. **analysis-files.csv** - 文件级别汇总
   - 列：File, LOC, Complexity, Symbols, Dependencies

3. **analysis-symbols.csv** - 符号级别详情
   - 列：File, Symbol Type, Symbol Name, LOC, Complexity

### 统计报告

终端会显示：
- 复杂度分布直方图
- Top 10 最复杂文件
- 按符号类型的统计（平均复杂度、平均 LOC）

### 示例输出

```
🔍 Finding TypeScript files in: /path/to/packages
📊 Found 523 TypeScript files

⏳ Processing: 100.0% (523/523)

✅ Analysis complete!

📈 Statistics:
────────────────────────────────────────────────────────────
  Total files analyzed: 520
  Failed files: 3
  Average LOC per file: 145.3
  Average complexity: 0.234

📊 Complexity Distribution:
  0.0-0.2: ████████████████████ 210 (40.4%)
  0.2-0.4: ████████████████ 165 (31.7%)
  0.4-0.6: ████████ 85 (16.3%)
  0.6-0.8: ████ 45 (8.7%)
  0.8-1.0: ██ 15 (2.9%)

🔥 Top 10 Most Complex Files:
  1. src/components/DataGrid/DataGrid.tsx
     Complexity: 0.847, LOC: 1250
  ...

🎯 Symbol Type Statistics:
  Type           Count     Avg Complexity    Avg LOC
  ──────────────────────────────────────────────────
  function       2341      0.345             28.5
  class          156       0.523             125.3
  interface      892       0.187             8.2
  type           1456      0.156             5.1
  ...
```

### 数据分析

生成的 CSV 文件可以导入 Excel/Google Sheets 进行进一步分析：
- 绘制复杂度与 LOC 的散点图
- 分析不同符号类型的复杂度分布
- 识别需要重构的候选文件

### 自定义 entrota 路径

如果 entrota 不在默认位置：

```bash
bun scripts/analyze-batch.ts /path/to/packages ./path/to/entrota
```
