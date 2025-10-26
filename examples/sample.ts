import { readFile } from 'fs/promises';
import { join } from 'path';

interface User {
  id: number;
  name: string;
  email: string;
}

type UserRole = 'admin' | 'user' | 'guest';

class UserService {
  private users: Map<number, User>;

  constructor() {
    this.users = new Map();
  }

  addUser(user: User): void {
    this.users.set(user.id, user);
  }

  getUser(id: number): User | undefined {
    return this.users.get(id);
  }

  deleteUser(id: number): boolean {
    return this.users.delete(id);
  }
}

function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

async function loadUsers(filePath: string): Promise<User[]> {
  const content = await readFile(filePath, 'utf-8');
  return JSON.parse(content);
}

enum Permission {
  Read = 'read',
  Write = 'write',
  Delete = 'delete',
}

export { UserService, validateEmail, loadUsers, Permission };
export type { User, UserRole };
