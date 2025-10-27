/**
 * Test Case 2: Function Declarations (Should Work Well)
 *
 * Expected Detection:
 * - 1 interface
 * - 1 type
 * - 5 functions with complexity
 *
 * This should work 100% correctly
 */

interface Config {
  maxRetries: number
  timeout: number
}

type Result<T> = {
  success: boolean
  data?: T
  error?: string
}

// Simple function - Complexity: 1
function add(a: number, b: number): number {
  return a + b
}

// Moderate complexity - Complexity: 4
function validateConfig(config: Config): boolean {
  if (!config) return false                          // +1

  if (config.maxRetries < 0) return false            // +1

  if (config.timeout <= 0) return false              // +1

  if (config.timeout > 60000) return false           // +1

  return true
}

// High complexity - Complexity: 10
function processData<T>(
  data: T[],
  filter?: (item: T) => boolean,
  transform?: (item: T) => T
): Result<T[]> {
  try {                                               // +1 (catch)
    if (!data || !Array.isArray(data)) {             // +1 (||)
      return { success: false, error: 'Invalid data' }
    }

    let result: T[] = []

    for (let i = 0; i < data.length; i++) {          // +1
      const item = data[i]

      if (!item) continue                             // +1

      if (filter && !filter(item)) {                  // +1 (&&)
        continue
      }

      if (transform) {                                // +1
        result.push(transform(item))
      } else {
        result.push(item)
      }
    }

    return { success: true, data: result }
  } catch (error) {                                   // counted above
    return { success: false, error: String(error) }
  }
}

// Nested function declarations
function createProcessor(config: Config) {
  // Inner function (may not be detected)
  function validate(data: any): boolean {
    if (!data) return false                           // +1
    if (typeof data !== 'object') return false        // +1
    return true                                       // Complexity: 3
  }

  return {
    process: (data: any) => validate(data)
  }
}

// Function with switch - Complexity: 6
function handleCommand(cmd: string): string {
  switch (cmd) {
    case 'start':                                     // +1
      return 'Starting...'
    case 'stop':                                      // +1
      return 'Stopping...'
    case 'pause':                                     // +1
      return 'Pausing...'
    case 'resume':                                    // +1
      return 'Resuming...'
    case 'reset':                                     // +1
      return 'Resetting...'
    default:
      return 'Unknown command'
  }
}
