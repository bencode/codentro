# Codescope Test Suite

A systematic test suite to verify codescope's feature coverage and accuracy.

## Quick Start

```bash
# Build the project
cd ..
cargo build --release

# Run tests
cd test-suite
./run-tests.sh

# View results
cat TEST_RESULTS.md
cat FINDINGS.md
```

## Test Files

### 01-arrow-functions.tsx
Tests arrow function detection (most common pattern in modern React/TypeScript)

**Expected:** 5 arrow functions
**Actual:** 0 ❌
**Status:** Not supported

### 02-function-declarations.ts
Tests standard function declarations (traditional TypeScript pattern)

**Expected:** 5 functions
**Actual:** 6 ✅ (includes nested function)
**Status:** Mostly supported, complexity accurate

### 03-class-methods.ts
Tests class method detection (OOP pattern)

**Expected:** 1 class + 7 methods
**Actual:** 1 class, 0 methods ❌
**Status:** Only detects class, not methods

### 04-function-expressions.ts
Tests function expressions (including IIFE)

**Expected:** 5 function expressions
**Actual:** 0 ❌
**Status:** Not supported

### 05-edge-cases.tsx
Tests edge cases and complex scenarios

**Expected:** 4 functions (mixed patterns)
**Actual:** 1 ⚠️
**Status:** Partial support, new issues found

## Key Findings

### ✅ What Works
- Standard function declarations (`function foo() {}`)
- Type definitions (type, interface, enum)
- Class declarations (class)
- Cyclomatic complexity calculation (for detected functions)
- File-level metrics (LOC, coupling)

### ❌ Not Supported
- Arrow functions (`const foo = () => {}`) - **Affects 70-80% of modern projects**
- Function expressions (`const foo = function() {}`)
- Class methods (including constructor, getter, static)
- Object methods

### ⚠️ Partial Support / Issues
- Nested functions (sometimes detected)
- Function declarations in complex files (may be missed)
- Logical operator complexity counting (occasionally off by ±1)

## Detection Rate Statistics

| Project Type | Function Detection | File Metrics | Overall Usability |
|--------------|-------------------|--------------|-------------------|
| Traditional TypeScript (function) | 80-90% | 90% | ⭐⭐⭐⭐ |
| Modern React/TS (arrow) | 20-30% | 90% | ⭐⭐ |
| OOP (class methods) | 0% | 90% | ⭐⭐ |
| Mixed Projects | 40-60% | 90% | ⭐⭐⭐ |

## Test Result Files

- **TEST_RESULTS.md** - Detailed test output (auto-generated)
- **FINDINGS.md** - In-depth analysis and issue summary
- **../ACCURACY_REPORT.md** - Complete accuracy report

## Priority Issues

### P0 - Critical (Blocks 70% of use cases)
1. Arrow function support

### P1 - High (Blocks 30% of use cases)
2. Function expression support
3. Class method detection
4. Investigate missing function declarations

### P2 - Medium
5. Improve logical operator complexity calculation
6. Enhance nested function detection reliability

## Development Recommendations

1. **Immediate Investigation**: Why are `complexConditions` and `createValidator` in test-05 not detected?
2. **Incremental Fix**: Start with arrow functions (biggest impact)
3. **Continuous Testing**: Re-run this test suite after each fix
4. **Extend Tests**: Add more test cases based on newly discovered issues

## Usage Examples

```bash
# Test a single file
../target/release/codescope 01-arrow-functions.tsx

# JSON output for details
../target/release/codescope 02-function-declarations.ts -f json | jq

# View detected functions
../target/release/codescope 02-function-declarations.ts -f json | \
  jq '.symbols[] | select(.kind == "function") | {name, loc, complexity}'
```

## Contributing Test Cases

If you discover new edge cases or issues:

1. Create a new test file in `test-suite/`
2. File naming format: `XX-description.ts(x)`
3. Add comments explaining expected detection results
4. Update `run-tests.sh` to add the new test
5. Update this README
