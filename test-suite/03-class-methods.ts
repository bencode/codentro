/**
 * Test Case 3: Class Methods
 *
 * Expected Detection:
 * - 1 class
 * - 6 methods (constructor + 5 methods)
 *
 * Current Detection:
 * - Only the class itself, NO methods
 */

class DataProcessor {
  private data: any[]
  private config: { maxSize: number; validateOnAdd: boolean }

  constructor(config?: { maxSize?: number; validateOnAdd?: boolean }) {
    this.data = []
    this.config = {
      maxSize: config?.maxSize || 100,
      validateOnAdd: config?.validateOnAdd ?? true
    }
  }

  // Method - Complexity: 4 (NOT DETECTED AS SEPARATE SYMBOL)
  add(item: any): boolean {
    if (this.config.validateOnAdd && !this.validate(item)) {  // +1 (&&)
      return false
    }

    if (this.data.length >= this.config.maxSize) {             // +1
      return false
    }

    this.data.push(item)
    return true
  }

  // Method - Complexity: 3 (NOT DETECTED)
  private validate(item: any): boolean {
    if (!item) return false                                    // +1
    if (typeof item !== 'object') return false                 // +1
    return true
  }

  // Method - Complexity: 7 (NOT DETECTED)
  process(filter?: (item: any) => boolean): any[] {
    const result = []

    for (let i = 0; i < this.data.length; i++) {              // +1
      const item = this.data[i]

      if (!item) continue                                      // +1

      if (filter && !filter(item)) {                           // +1 (&&)
        continue
      }

      if (item.processed) {                                    // +1
        result.push(item)
      } else {
        const processed = this.transform(item)
        if (processed) {                                       // +1
          result.push(processed)
        }
      }
    }

    return result
  }

  // Method - Complexity: 2 (NOT DETECTED)
  private transform(item: any): any {
    if (item.value === undefined) {                            // +1
      return null
    }
    return { ...item, processed: true }
  }

  // Getter (NOT DETECTED)
  get size(): number {
    return this.data.length
  }

  // Static method (NOT DETECTED) - Complexity: 5
  static create(options?: any): DataProcessor {
    if (!options) {                                            // +1
      return new DataProcessor()
    }

    if (options.type === 'large') {                            // +1
      return new DataProcessor({ maxSize: 1000 })
    } else if (options.type === 'small') {                     // +1
      return new DataProcessor({ maxSize: 10 })
    } else {
      return new DataProcessor({ maxSize: 100 })
    }
  }
}

export default DataProcessor
