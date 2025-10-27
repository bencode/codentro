#!/usr/bin/env bun

import { readdirSync, statSync, writeFileSync } from 'fs';
import { join, relative } from 'path';
import { execSync } from 'child_process';

interface QualityMetric {
  name: string;
  value: number;
  threshold?: number;
  severity: 'info' | 'warning' | 'error';
  message?: string;
}

interface AnalysisResult {
  path: string;
  loc: number;
  comment_lines: number;
  blank_lines: number;
  symbols: Array<{
    kind: string;
    name: string;
    loc: number;
    metrics: QualityMetric[];
  }>;
  metrics: QualityMetric[];
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

// 运行 codescope 分析单个文件
function analyzeFile(filePath: string, codescopeBin: string): AnalysisResult | null {
  try {
    const output = execSync(`${codescopeBin} view "${filePath}" --format json`, {
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
    console.error('Usage: bun analyze-batch.ts <target-directory> [codescope-binary-path]');
    console.error('');
    console.error('Example:');
    console.error('  bun analyze-batch.ts /path/to/packages');
    console.error('  bun analyze-batch.ts /path/to/packages ./target/release/codescope');
    process.exit(1);
  }

  const targetDir = args[0];
  const codescopeBin = args[1] || './target/release/codescope';

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
      const result = analyzeFile(file, codescopeBin);
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
  avgCommentLines: number;
  avgBlankLines: number;
  largeFilesCount: number;
  topLargeFiles: Array<{ path: string; loc: number; warnings: number }>;
  symbolTypeStats: Record<string, { count: number; avgLOC: number; issues: number }>;
  metricSummary: Record<string, { count: number; avgValue: number; violations: number }>;
}

function calculateStatistics(results: AnalysisResult[]): Statistics {
  const totalLOC = results.reduce((sum, r) => sum + r.loc, 0);
  const totalComment = results.reduce((sum, r) => sum + r.comment_lines, 0);
  const totalBlank = results.reduce((sum, r) => sum + r.blank_lines, 0);

  // Count files with warnings
  const filesWithWarnings = results.filter(
    (r) => r.metrics.some((m) => m.severity === 'warning' || m.severity === 'error')
  );

  // Top large files
  const topLargeFiles = results
    .map((r) => ({
      path: r.path,
      loc: r.loc,
      warnings: r.metrics.filter((m) => m.severity === 'warning' || m.severity === 'error').length,
    }))
    .sort((a, b) => b.loc - a.loc)
    .slice(0, 10);

  // Symbol type stats
  const symbolTypeStats: Record<string, { count: number; totalLOC: number; issues: number }> = {};

  for (const result of results) {
    for (const symbol of result.symbols) {
      if (!symbolTypeStats[symbol.kind]) {
        symbolTypeStats[symbol.kind] = { count: 0, totalLOC: 0, issues: 0 };
      }
      symbolTypeStats[symbol.kind].count++;
      symbolTypeStats[symbol.kind].totalLOC += symbol.loc;
      const hasIssues = symbol.metrics.some(
        (m) => m.severity === 'warning' || m.severity === 'error'
      );
      if (hasIssues) {
        symbolTypeStats[symbol.kind].issues++;
      }
    }
  }

  const symbolTypeStatsFormatted = Object.fromEntries(
    Object.entries(symbolTypeStats).map(([kind, stats]) => [
      kind,
      {
        count: stats.count,
        avgLOC: stats.totalLOC / stats.count,
        issues: stats.issues,
      },
    ])
  );

  // Metric summary
  const metricSummary: Record<string, { count: number; totalValue: number; violations: number }> = {};

  for (const result of results) {
    for (const metric of result.metrics) {
      if (!metricSummary[metric.name]) {
        metricSummary[metric.name] = { count: 0, totalValue: 0, violations: 0 };
      }
      metricSummary[metric.name].count++;
      metricSummary[metric.name].totalValue += metric.value;
      if (metric.severity === 'warning' || metric.severity === 'error') {
        metricSummary[metric.name].violations++;
      }
    }
  }

  const metricSummaryFormatted = Object.fromEntries(
    Object.entries(metricSummary).map(([name, stats]) => [
      name,
      {
        count: stats.count,
        avgValue: stats.totalValue / stats.count,
        violations: stats.violations,
      },
    ])
  );

  return {
    avgLOC: totalLOC / results.length,
    avgCommentLines: totalComment / results.length,
    avgBlankLines: totalBlank / results.length,
    largeFilesCount: filesWithWarnings.length,
    topLargeFiles,
    symbolTypeStats: symbolTypeStatsFormatted,
    metricSummary: metricSummaryFormatted,
  };
}

function printStatistics(stats: Statistics, batch: BatchResult) {
  console.log('📈 Statistics:');
  console.log('─'.repeat(60));
  console.log(`  Total files analyzed: ${batch.analyzedFiles}`);
  console.log(`  Failed files: ${batch.failedFiles}`);
  console.log(`  Files with warnings: ${stats.largeFilesCount}`);
  console.log(`  Average LOC per file: ${stats.avgLOC.toFixed(1)}`);
  console.log(`  Average comment lines: ${stats.avgCommentLines.toFixed(1)}`);
  console.log(`  Average blank lines: ${stats.avgBlankLines.toFixed(1)}`);
  console.log('');

  console.log('📊 Quality Metrics Summary:');
  console.log('  Metric'.padEnd(30) + 'Avg Value'.padEnd(15) + 'Violations');
  console.log('  ' + '─'.repeat(55));
  Object.entries(stats.metricSummary)
    .sort((a, b) => b[1].violations - a[1].violations)
    .forEach(([name, stat]) => {
      console.log(
        `  ${name.padEnd(30)}${stat.avgValue.toFixed(1).padEnd(15)}${stat.violations}`
      );
    });
  console.log('');

  console.log('🔥 Top 10 Largest Files:');
  stats.topLargeFiles.forEach((file, idx) => {
    const warningIcon = file.warnings > 0 ? '⚠' : ' ';
    console.log(`  ${idx + 1}. ${warningIcon} ${file.path}`);
    console.log(`     LOC: ${file.loc}, Warnings: ${file.warnings}`);
  });
  console.log('');

  console.log('🎯 Symbol Type Statistics:');
  console.log('  Type'.padEnd(15) + 'Count'.padEnd(10) + 'Avg LOC'.padEnd(12) + 'Issues');
  console.log('  ' + '─'.repeat(50));
  Object.entries(stats.symbolTypeStats)
    .sort((a, b) => b[1].count - a[1].count)
    .forEach(([kind, stat]) => {
      console.log(
        `  ${kind.padEnd(15)}${String(stat.count).padEnd(10)}${stat.avgLOC.toFixed(1).padEnd(12)}${stat.issues}`
      );
    });
}

function generateCSVReport(results: AnalysisResult[]) {
  const rows = [
    ['File', 'LOC', 'Comment', 'Blank', 'Symbols', 'Dependencies', 'Warnings'].join(','),
  ];

  for (const result of results) {
    const warnings = result.metrics.filter(
      (m) => m.severity === 'warning' || m.severity === 'error'
    ).length;

    rows.push(
      [
        `"${result.path}"`,
        result.loc,
        result.comment_lines,
        result.blank_lines,
        result.symbols.length,
        result.outgoing.length,
        warnings,
      ].join(',')
    );
  }

  // Symbol-level CSV
  const symbolRows = [
    ['File', 'Symbol Type', 'Symbol Name', 'LOC', 'Issues'].join(','),
  ];

  for (const result of results) {
    for (const symbol of result.symbols) {
      const issues = symbol.metrics.filter(
        (m) => m.severity === 'warning' || m.severity === 'error'
      ).length;

      symbolRows.push(
        [
          `"${result.path}"`,
          symbol.kind,
          `"${symbol.name}"`,
          symbol.loc,
          issues,
        ].join(',')
      );
    }
  }

  // Metrics CSV
  const metricRows = [
    ['File', 'Metric Name', 'Value', 'Threshold', 'Severity', 'Message'].join(','),
  ];

  for (const result of results) {
    for (const metric of result.metrics) {
      metricRows.push(
        [
          `"${result.path}"`,
          metric.name,
          metric.value.toFixed(1),
          metric.threshold?.toFixed(1) || '-',
          metric.severity,
          `"${metric.message || ''}"`,
        ].join(',')
      );
    }
  }

  writeFileSync('analysis-files.csv', rows.join('\n'));
  writeFileSync('analysis-symbols.csv', symbolRows.join('\n'));
  writeFileSync('analysis-metrics.csv', metricRows.join('\n'));

  console.log('\n📄 CSV reports generated:');
  console.log('  - analysis-files.csv');
  console.log('  - analysis-symbols.csv');
  console.log('  - analysis-metrics.csv');
}

main().catch((error) => {
  console.error('❌ Error:', error);
  process.exit(1);
});
