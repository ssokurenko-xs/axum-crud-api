import { useState } from 'react';
import type { User } from './api';
import { useUsers } from './useUsers';
import { UserForm } from './UserForm';
import { UserTable } from './UserTable';

function App() {
  const { users, loading, error, add, update, remove } = useUsers();
  const [editing, setEditing] = useState<User | null>(null);

  const handleSubmit = async (payload: { name: string; email: string }) => {
    if (editing) {
      await update(editing.id, payload);
      setEditing(null);
    } else {
      await add(payload);
    }
  };

  return (
    <div className="min-h-screen bg-base-200">
      <div className="container mx-auto max-w-3xl p-6">
        <h1 className="text-3xl font-bold mb-6">Users</h1>

        <UserForm
          editing={editing}
          onSubmit={handleSubmit}
          onCancel={() => setEditing(null)}
        />

        {error && (
          <div role="alert" className="alert alert-error mb-4">
            <span>{error}</span>
          </div>
        )}

        {loading ? (
          <div className="flex justify-center py-12">
            <span className="loading loading-spinner loading-lg"></span>
          </div>
        ) : (
          <UserTable
            users={users}
            onEdit={setEditing}
            onDelete={remove}
          />
        )}
      </div>
    </div>
  );
}

export default App;
