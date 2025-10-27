/**
 * Test Case 5: Edge Cases & Complex Scenarios
 *
 * Tests for:
 * - Nested functions
 * - Multiple function types in one file
 * - Complex logical expressions
 * - React hooks and patterns
 */

import { useState, useEffect, useCallback } from 'react'

type Todo = {
  id: number
  text: string
  completed: boolean
}

// Regular function declaration - SHOULD BE DETECTED
function createTodo(text: string): Todo {
  return {
    id: Date.now(),
    text,
    completed: false
  }
}

// React hook (arrow function) - NOT DETECTED - Complexity: 8
const useTodos = (initialTodos: Todo[] = []) => {
  const [todos, setTodos] = useState<Todo[]>(initialTodos)
  const [filter, setFilter] = useState<'all' | 'active' | 'completed'>('all')

  const addTodo = useCallback((text: string) => {
    if (!text || !text.trim()) return                         // +1 (||)

    setTodos(prev => [...prev, createTodo(text)])
  }, [])

  const toggleTodo = useCallback((id: number) => {
    setTodos(prev =>
      prev.map(todo =>
        todo.id === id ? { ...todo, completed: !todo.completed } : todo
      )
    )
  }, [])

  const filteredTodos = todos.filter(todo => {
    if (filter === 'all') return true                         // +1
    if (filter === 'active') return !todo.completed          // +1
    return todo.completed
  })

  return { todos: filteredTodos, addTodo, toggleTodo, setFilter }
}

// Component with nested arrow functions - NOT DETECTED
const TodoApp = () => {
  const { todos, addTodo, toggleTodo, setFilter } = useTodos()
  const [inputValue, setInputValue] = useState('')

  // Nested arrow function - NOT DETECTED - Complexity: 2
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (inputValue.trim()) {                                  // +1
      addTodo(inputValue)
      setInputValue('')
    }
  }

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <input
          value={inputValue}
          onChange={e => setInputValue(e.target.value)}
        />
      </form>
      {todos.map(todo => (
        <div key={todo.id} onClick={() => toggleTodo(todo.id)}>
          {todo.text}
        </div>
      ))}
    </div>
  )
}

// Complex logical expressions - SHOULD BE DETECTED - Complexity: 13
function complexConditions(data: any, options: any): boolean {
  if (!data) return false                                     // +1

  if (
    (options.checkType && typeof data !== 'object') ||        // +1 (&&), +1 (||)
    (options.checkNull && data === null) ||                   // +1 (&&), +1 (||)
    (options.checkArray && Array.isArray(data))               // +1 (&&)
  ) {
    return false
  }

  if (options.checkKeys) {                                    // +1
    for (const key in data) {                                 // +1
      if (
        !options.allowedKeys ||                               // +1 (||)
        !options.allowedKeys.includes(key)
      ) {
        return false
      }
    }
  }

  if (options.validate) {                                     // +1
    try {                                                     // +1 (catch)
      return options.validate(data)
    } catch (e) {
      return false
    }
  }

  return true
}

// Higher-order function - SHOULD BE DETECTED - Complexity: 3
function createValidator(rules: any[]) {
  // Inner function - may not be detected
  function validate(value: any): boolean {
    for (const rule of rules) {                               // +1
      if (!rule.check(value)) {                               // +1
        return false
      }
    }
    return true
  }

  return validate
}

export default TodoApp
