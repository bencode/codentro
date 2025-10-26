#!/usr/bin/env bun

import { readdirSync, statSync, writeFileSync } from 'fs';
import { join, relative } from 'path';
import { execSync } from 'child_process';

interface AnalysisResult {
  path: string;
  loc: number;
  complexity: number;
  symbols: Array<{
    kind: string;
    name: string;
    loc: number;
    complexity: number;
  }>;
  outgoing: Array<{
    target: string;
    relation: string;
    strength: number;
  }>;
}

interface BatchResult {
  totalFiles: number;
  analyzedFiles: number;
  failedFiles: number;
  results: AnalysisResult[];
  errors: Array<{ path: string; error: string }>;
}

// 递归查找所有 .ts 文件（排除 .d.ts 和特殊目录）
function findTypeScriptFiles(dir: string, baseDir: string): string[] {
  const files: string[] = [];
  const skipDirs = ['node_modules', '.git', 'dist', 'build', '__tests__', 'test'];

  try {
    const entries = readdirSync(dir, { withFileTypes: true });

    for (const entry of entries) {
      const fullPath = join(dir, entry.name);

      if (entry.isDirectory()) {
        if (!skipDirs.includes(entry.name)) {
          files.push(...findTypeScriptFiles(fullPath, baseDir));
        }
      } else if (entry.isFile() && entry.name.endsWith('.ts') && !entry.name.endsWith('.d.ts')) {
        files.push(relative(baseDir, fullPath));
      }
    }
  } catch (error) {
    console.error(`Error reading directory ${dir}:`, error);
  }

  return files;
}

// 运行 entrota 分析单个文件
function analyzeFile(filePath: string, entrotaBin: string): AnalysisResult | null {
  try {
    const output = execSync(`${entrotaBin} view "${filePath}" --format json`, {
      encoding: 'utf-8',
      maxBuffer: 10 * 1024 * 1024, // 10MB buffer
    });

    return JSON.parse(output);
  } catch (error: any) {
    throw new Error(error.stderr || error.message);
  }
}

// 主函数
async function main() {
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.error('Usage: bun analyze-batch.ts <target-directory> [entrota-binary-path]');
    console.error('');
    console.error('Example:');
    console.error('  bun analyze-batch.ts /path/to/packages');
    console.error('  bun analyze-batch.ts /path/to/packages ./target/release/entrota');
    process.exit(1);
  }

  const targetDir = args[0];
  const entrotaBin = args[1] || './target/release/entrota';

  console.log(`🔍 Finding TypeScript files in: ${targetDir}`);
  const tsFiles = findTypeScriptFiles(targetDir, process.cwd());

  console.log(`📊 Found ${tsFiles.length} TypeScript files`);
  console.log('');

  const batchResult: BatchResult = {
    totalFiles: tsFiles.length,
    analyzedFiles: 0,
    failedFiles: 0,
    results: [],
    errors: [],
  };

  let processed = 0;

  for (const file of tsFiles) {
    processed++;
    const percentage = ((processed / tsFiles.length) * 100).toFixed(1);
    process.stdout.write(`\r⏳ Processing: ${percentage}% (${processed}/${tsFiles.length})`);

    try {
      const result = analyzeFile(file, entrotaBin);
      if (result) {
        batchResult.results.push(result);
        batchResult.analyzedFiles++;
      }
    } catch (error: any) {
      batchResult.failedFiles++;
      batchResult.errors.push({
        path: file,
        error: error.message,
      });
    }
  }

  console.log('\n');
  console.log('✅ Analysis complete!');
  console.log('');

  // 统计信息
  const stats = calculateStatistics(batchResult.results);
  printStatistics(stats, batchResult);

  // 保存结果到文件
  const outputFile = 'analysis-results.json';
  writeFileSync(outputFile, JSON.stringify(batchResult, null, 2));
  console.log(`\n💾 Full results saved to: ${outputFile}`);

  // 生成 CSV 报告
  generateCSVReport(batchResult.results);
}

interface Statistics {
  avgLOC: number;
  avgComplexity: number;
  complexityDistribution: { range: string; count: number }[];
  topComplexFiles: Array<{ path: string; complexity: number; loc: number }>;
  topComplexSymbols: Array<{ file: string; symbol: string; complexity: number; loc: number }>;
  symbolTypeStats: Record<string, { count: number; avgComplexity: number; avgLOC: number }>;
}

function calculateStatistics(results: AnalysisResult[]): Statistics {
  const totalLOC = results.reduce((sum, r) => sum + r.loc, 0);
  const totalComplexity = results.reduce((sum, r) => sum + r.complexity, 0);

  // 复杂度分布
  const ranges = [
    { range: '0.0-0.2', min: 0, max: 0.2 },
    { range: '0.2-0.4', min: 0.2, max: 0.4 },
    { range: '0.4-0.6', min: 0.4, max: 0.6 },
    { range: '0.6-0.8', min: 0.6, max: 0.8 },
    { range: '0.8-1.0', min: 0.8, max: 1.0 },
  ];

  const distribution = ranges.map(({ range, min, max }) => ({
    range,
    count: results.filter((r) => r.complexity >= min && r.complexity < max).length,
  }));

  // Top 复杂文件
  const topComplexFiles = results
    .map((r) => ({ path: r.path, complexity: r.complexity, loc: r.loc }))
    .sort((a, b) => b.complexity - a.complexity)
    .slice(0, 10);

  // Top 复杂符号
  const allSymbols = results.flatMap((r) =>
    r.symbols.map((s) => ({
      file: r.path,
      symbol: `${s.kind}:${s.name}`,
      complexity: s.complexity || 0,
      loc: s.loc,
    }))
  );

  const topComplexSymbols = allSymbols
    .sort((a, b) => b.complexity - a.complexity)
    .slice(0, 20);

  // 按符号类型统计
  const symbolTypeStats: Record<string, { count: number; totalComplexity: number; totalLOC: number }> = {};

  for (const result of results) {
    for (const symbol of result.symbols) {
      if (!symbolTypeStats[symbol.kind]) {
        symbolTypeStats[symbol.kind] = { count: 0, totalComplexity: 0, totalLOC: 0 };
      }
      symbolTypeStats[symbol.kind].count++;
      symbolTypeStats[symbol.kind].totalComplexity += symbol.complexity || 0;
      symbolTypeStats[symbol.kind].totalLOC += symbol.loc;
    }
  }

  const symbolTypeStatsFormatted = Object.fromEntries(
    Object.entries(symbolTypeStats).map(([kind, stats]) => [
      kind,
      {
        count: stats.count,
        avgComplexity: stats.totalComplexity / stats.count,
        avgLOC: stats.totalLOC / stats.count,
      },
    ])
  );

  return {
    avgLOC: totalLOC / results.length,
    avgComplexity: totalComplexity / results.length,
    complexityDistribution: distribution,
    topComplexFiles,
    topComplexSymbols,
    symbolTypeStats: symbolTypeStatsFormatted,
  };
}

function printStatistics(stats: Statistics, batch: BatchResult) {
  console.log('📈 Statistics:');
  console.log('─'.repeat(60));
  console.log(`  Total files analyzed: ${batch.analyzedFiles}`);
  console.log(`  Failed files: ${batch.failedFiles}`);
  console.log(`  Average LOC per file: ${stats.avgLOC.toFixed(1)}`);
  console.log(`  Average complexity: ${stats.avgComplexity.toFixed(3)}`);
  console.log('');

  console.log('📊 Complexity Distribution:');
  stats.complexityDistribution.forEach(({ range, count }) => {
    const bar = '█'.repeat(Math.floor((count / batch.analyzedFiles) * 50));
    const percentage = ((count / batch.analyzedFiles) * 100).toFixed(1);
    console.log(`  ${range}: ${bar} ${count} (${percentage}%)`);
  });
  console.log('');

  console.log('🔥 Top 10 Most Complex Files:');
  stats.topComplexFiles.forEach((file, idx) => {
    console.log(`  ${idx + 1}. ${file.path}`);
    console.log(`     Complexity: ${file.complexity.toFixed(3)}, LOC: ${file.loc}`);
  });
  console.log('');

  console.log('🎯 Symbol Type Statistics:');
  console.log('  Type'.padEnd(15) + 'Count'.padEnd(10) + 'Avg Complexity'.padEnd(18) + 'Avg LOC');
  console.log('  ' + '─'.repeat(50));
  Object.entries(stats.symbolTypeStats)
    .sort((a, b) => b[1].count - a[1].count)
    .forEach(([kind, stat]) => {
      console.log(
        `  ${kind.padEnd(15)}${String(stat.count).padEnd(10)}${stat.avgComplexity.toFixed(3).padEnd(18)}${stat.avgLOC.toFixed(1)}`
      );
    });
}

function generateCSVReport(results: AnalysisResult[]) {
  const rows = [
    ['File', 'LOC', 'Complexity', 'Symbols', 'Dependencies'].join(','),
  ];

  for (const result of results) {
    rows.push(
      [
        `"${result.path}"`,
        result.loc,
        result.complexity.toFixed(3),
        result.symbols.length,
        result.outgoing.length,
      ].join(',')
    );
  }

  // 符号级别的 CSV
  const symbolRows = [
    ['File', 'Symbol Type', 'Symbol Name', 'LOC', 'Complexity'].join(','),
  ];

  for (const result of results) {
    for (const symbol of result.symbols) {
      symbolRows.push(
        [
          `"${result.path}"`,
          symbol.kind,
          `"${symbol.name}"`,
          symbol.loc,
          (symbol.complexity || 0).toFixed(3),
        ].join(',')
      );
    }
  }

  writeFileSync('analysis-files.csv', rows.join('\n'));
  writeFileSync('analysis-symbols.csv', symbolRows.join('\n'));

  console.log('\n📄 CSV reports generated:');
  console.log('  - analysis-files.csv');
  console.log('  - analysis-symbols.csv');
}

main().catch((error) => {
  console.error('❌ Error:', error);
  process.exit(1);
});
