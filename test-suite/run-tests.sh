#!/bin/bash
# Test Suite Runner for Codescope

BINARY="../target/release/codescope"
REPORT="TEST_RESULTS.md"

echo "# Codescope Test Suite Results" > $REPORT
echo "" >> $REPORT
echo "**Date:** $(date)" >> $REPORT
echo "**Version:** v0.1" >> $REPORT
echo "" >> $REPORT

run_test() {
    local file=$1
    local name=$2
    local expected_functions=$3
    local expected_types=$4

    echo "## Test: $name" >> $REPORT
    echo "" >> $REPORT
    echo "**File:** \`$file\`" >> $REPORT
    echo "" >> $REPORT

    # Run analysis
    output=$($BINARY "$file" 2>&1)
    json=$($BINARY "$file" -f json 2>&1)

    # Extract counts
    function_count=$(echo "$json" | jq '[.symbols[] | select(.kind == "function")] | length' 2>/dev/null || echo 0)
    type_count=$(echo "$json" | jq '[.symbols[] | select(.kind == "type")] | length' 2>/dev/null || echo 0)
    class_count=$(echo "$json" | jq '[.symbols[] | select(.kind == "class")] | length' 2>/dev/null || echo 0)
    interface_count=$(echo "$json" | jq '[.symbols[] | select(.kind == "interface")] | length' 2>/dev/null || echo 0)

    echo "### Expected vs Actual" >> $REPORT
    echo "" >> $REPORT
    echo "| Metric | Expected | Actual | Status |" >> $REPORT
    echo "|--------|----------|--------|--------|" >> $REPORT
    echo "| Functions | $expected_functions | $function_count | $([ $function_count -eq $expected_functions ] && echo '✅' || echo '❌') |" >> $REPORT
    echo "| Types | $expected_types | $type_count | $([ $type_count -eq $expected_types ] && echo '✅' || echo '❌') |" >> $REPORT
    echo "| Classes | - | $class_count | - |" >> $REPORT
    echo "| Interfaces | - | $interface_count | - |" >> $REPORT
    echo "" >> $REPORT

    echo "### Detected Symbols" >> $REPORT
    echo "" >> $REPORT
    echo '```json' >> $REPORT
    echo "$json" | jq '.symbols | map({kind, name, loc, complexity: .cyclomatic_complexity})' >> $REPORT
    echo '```' >> $REPORT
    echo "" >> $REPORT

    echo "---" >> $REPORT
    echo "" >> $REPORT
}

echo "Running test suite..."

run_test "01-arrow-functions.tsx" "Arrow Functions" 5 2
run_test "02-function-declarations.ts" "Function Declarations" 5 2
run_test "03-class-methods.ts" "Class Methods" 7 0
run_test "04-function-expressions.ts" "Function Expressions" 5 0
run_test "05-edge-cases.tsx" "Edge Cases & Complex Scenarios" 4 1

echo "" >> $REPORT
echo "## Summary" >> $REPORT
echo "" >> $REPORT
echo "Test suite completed. See detailed results above." >> $REPORT

echo "Test suite completed! Results saved to $REPORT"
