/**
 * Test Case 4: Function Expressions
 *
 * Expected Detection:
 * - 5 function expressions
 *
 * Current Detection:
 * - NONE (function expressions not supported)
 */

// Function expression - Complexity: 1 (NOT DETECTED)
const greet = function(name: string): string {
  return `Hello, ${name}!`
}

// Named function expression - Complexity: 3 (NOT DETECTED)
const factorial = function fact(n: number): number {
  if (n <= 0) return 1                                // +1
  if (n === 1) return 1                               // +1
  return n * fact(n - 1)
}

// Function expression with high complexity - Complexity: 9 (NOT DETECTED)
const complexValidator = function(data: any, rules: any): boolean {
  if (!data) return false                             // +1

  if (!rules || typeof rules !== 'object') {          // +1 (||)
    return false
  }

  for (const key in rules) {                          // +1
    const rule = rules[key]
    const value = data[key]

    if (!rule) continue                               // +1

    if (rule.required && !value) {                    // +1 (&&)
      return false
    }

    if (rule.type && typeof value !== rule.type) {    // +1 (&&)
      return false
    }

    if (rule.min !== undefined && value < rule.min) { // +1 (&&)
      return false
    }

    if (rule.max !== undefined && value > rule.max) { // +1 (&&)
      return false
    }
  }

  return true
}

// IIFE (Immediately Invoked Function Expression) - NOT DETECTED
const config = (function() {
  const env = process.env.NODE_ENV || 'development'   // +1 (||)

  if (env === 'production') {                         // +1
    return { debug: false, timeout: 5000 }
  } else if (env === 'test') {                        // +1
    return { debug: true, timeout: 1000 }
  } else {
    return { debug: true, timeout: 3000 }
  }
})()

// Export default function expression - NOT DETECTED
export default function(data: any[]): any {
  if (!data || data.length === 0) {                   // +1 (||)
    return null
  }

  return data.filter(item => {
    if (!item) return false                           // +1
    return true
  })
}
