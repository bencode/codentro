/**
 * Test Case 1: Arrow Functions
 *
 * Expected Detection:
 * - 2 type definitions
 * - 5 arrow function components/utilities
 *
 * Current Detection:
 * - Only types, NO functions
 */

type User = {
  name: string
  age: number
}

type Props = {
  user: User
  onUpdate: (user: User) => void
}

// Arrow function component (NOT DETECTED)
const UserCard = ({ user, onUpdate }: Props) => {
  const handleClick = () => {
    onUpdate({ ...user, age: user.age + 1 })
  }

  if (!user) return null

  return (
    <div onClick={handleClick}>
      {user.name} - {user.age}
    </div>
  )
}

// Arrow function utility (NOT DETECTED)
const validateUser = (user: User) => {
  if (!user.name) return false
  if (user.age < 0 || user.age > 150) return false
  return true
}

// Complex arrow function (NOT DETECTED)
const processUsers = (users: User[], filter?: string) => {
  return users
    .filter(u => {
      if (!filter) return true
      return u.name.includes(filter)
    })
    .map(u => ({
      ...u,
      displayName: `${u.name} (${u.age})`
    }))
    .sort((a, b) => a.age - b.age)
}

// Arrow function with high complexity (NOT DETECTED)
const complexLogic = (data: any[], options: any) => {
  let result = []

  for (let i = 0; i < data.length; i++) {     // +1
    const item = data[i]

    if (!item) continue                        // +1

    if (options && options.filter) {           // +1 (&&)
      if (!options.filter(item)) continue      // +1
    }

    if (item.type === 'A') {                   // +1
      result.push(item.value * 2)
    } else if (item.type === 'B') {            // +1
      result.push(item.value * 3)
    } else {
      result.push(item.value)
    }
  }

  return result
}

export default UserCard
