import type { components } from './schema';

export type User = components['schemas']['User'];
export type UserPayload = components['schemas']['UserPayload'];

const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

async function handleResponse<T>(res: Response): Promise<T> {
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || res.statusText);
  }
  return res.json();
}

export async function getUsers(): Promise<User[]> {
  const res = await fetch(`${BACKEND_URL}/users`);
  return handleResponse(res);
}

export async function getUser(id: string): Promise<User> {
  const res = await fetch(`${BACKEND_URL}/users/${id}`);
  return handleResponse(res);
}

export async function createUser(payload: UserPayload): Promise<User> {
  const res = await fetch(`${BACKEND_URL}/users`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  return handleResponse(res);
}

export async function updateUser(id: string, payload: UserPayload): Promise<User> {
  const res = await fetch(`${BACKEND_URL}/users/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  return handleResponse(res);
}

export async function deleteUser(id: string): Promise<void> {
  const res = await fetch(`${BACKEND_URL}/users/${id}`, { method: 'DELETE' });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || res.statusText);
  }
}
