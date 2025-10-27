// Example demonstrating cyclomatic complexity analysis

// Simple function - Complexity: 1
export function add(a: number, b: number): number {
    return a + b;
}

// Moderate complexity - Complexity: 4
export function validateEmail(email: string): boolean {
    if (!email) return false;                    // +1

    if (!email.includes('@')) return false;       // +1

    const parts = email.split('@');
    if (parts.length !== 2) return false;         // +1

    if (!parts[1].includes('.')) return false;    // +1

    return true;
}

// High complexity - Complexity: 13
export function processUserData(user: any, options?: any): any {
    if (!user) return null;                       // +1

    const result: any = {};

    if (user.name && user.name.trim()) {          // +1 (&&)
        result.name = user.name.trim();
    }

    if (user.email) {                             // +1
        if (validateEmail(user.email)) {          // +1
            result.email = user.email;
        } else {
            throw new Error('Invalid email');
        }
    }

    if (options && options.includeAge) {          // +1 (&&)
        result.age = user.age || 0;               // +1 (||)
    }

    if (options && options.validateAge) {         // +1 (&&)
        if (user.age < 0 || user.age > 150) {     // +1 (||)
            throw new Error('Invalid age');
        }
    }

    for (let key in user.metadata || {}) {        // +1, +1 (||)
        if (key.startsWith('_')) continue;        // +1
        result[key] = user.metadata[key];
    }

    return result;
}
