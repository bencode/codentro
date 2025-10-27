# Codescope Test Suite Results

**Date:** Tue Oct 28 00:33:44 CST 2025
**Version:** v0.1

## Test: Arrow Functions

**File:** `01-arrow-functions.tsx`

### Expected vs Actual

| Metric | Expected | Actual | Status |
|--------|----------|--------|--------|
| Functions | 5 | 0 | ❌ |
| Types | 2 | 2 | ✅ |
| Classes | - | 0 | - |
| Interfaces | - | 0 | - |

### Detected Symbols

```json
[
  {
    "kind": "type",
    "name": "User",
    "loc": 4,
    "complexity": null
  },
  {
    "kind": "type",
    "name": "Props",
    "loc": 4,
    "complexity": null
  }
]
```

---

## Test: Function Declarations

**File:** `02-function-declarations.ts`

### Expected vs Actual

| Metric | Expected | Actual | Status |
|--------|----------|--------|--------|
| Functions | 5 | 6 | ❌ |
| Types | 2 | 1 | ❌ |
| Classes | - | 0 | - |
| Interfaces | - | 1 | - |

### Detected Symbols

```json
[
  {
    "kind": "interface",
    "name": "Config",
    "loc": 4,
    "complexity": null
  },
  {
    "kind": "type",
    "name": "Result",
    "loc": 5,
    "complexity": null
  },
  {
    "kind": "function",
    "name": "add",
    "loc": 3,
    "complexity": 1
  },
  {
    "kind": "function",
    "name": "validateConfig",
    "loc": 11,
    "complexity": 5
  },
  {
    "kind": "function",
    "name": "processData",
    "loc": 33,
    "complexity": 9
  },
  {
    "kind": "function",
    "name": "createProcessor",
    "loc": 12,
    "complexity": 3
  },
  {
    "kind": "function",
    "name": "validate",
    "loc": 5,
    "complexity": 3
  },
  {
    "kind": "function",
    "name": "handleCommand",
    "loc": 16,
    "complexity": 6
  }
]
```

---

## Test: Class Methods

**File:** `03-class-methods.ts`

### Expected vs Actual

| Metric | Expected | Actual | Status |
|--------|----------|--------|--------|
| Functions | 7 | 0 | ❌ |
| Types | 0 | 0 | ✅ |
| Classes | - | 1 | - |
| Interfaces | - | 0 | - |

### Detected Symbols

```json
[
  {
    "kind": "class",
    "name": "DataProcessor",
    "loc": 87,
    "complexity": null
  }
]
```

---

## Test: Function Expressions

**File:** `04-function-expressions.ts`

### Expected vs Actual

| Metric | Expected | Actual | Status |
|--------|----------|--------|--------|
| Functions | 5 | 0 | ❌ |
| Types | 0 | 0 | ✅ |
| Classes | - | 0 | - |
| Interfaces | - | 0 | - |

### Detected Symbols

```json
[]
```

---

## Test: Edge Cases & Complex Scenarios

**File:** `05-edge-cases.tsx`

### Expected vs Actual

| Metric | Expected | Actual | Status |
|--------|----------|--------|--------|
| Functions | 4 | 1 | ❌ |
| Types | 1 | 1 | ✅ |
| Classes | - | 0 | - |
| Interfaces | - | 0 | - |

### Detected Symbols

```json
[
  {
    "kind": "type",
    "name": "Todo",
    "loc": 5,
    "complexity": null
  },
  {
    "kind": "function",
    "name": "createTodo",
    "loc": 7,
    "complexity": 1
  }
]
```

---


## Summary

Test suite completed. See detailed results above.
