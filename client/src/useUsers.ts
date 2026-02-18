import { useCallback, useEffect, useState } from 'react';
import { type User, getUsers, createUser, updateUser, deleteUser, type UserPayload } from './api';

export function useUsers() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await getUsers();
      setUsers(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to fetch users');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { refresh(); }, [refresh]);

  const add = async (payload: UserPayload) => {
    await createUser(payload);
    await refresh();
  };

  const update = async (id: string, payload: UserPayload) => {
    await updateUser(id, payload);
    await refresh();
  };

  const remove = async (id: string) => {
    await deleteUser(id);
    await refresh();
  };

  return { users, loading, error, refresh, add, update, remove };
}
