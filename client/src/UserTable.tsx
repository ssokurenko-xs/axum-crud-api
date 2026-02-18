import type { User } from './api';

interface Props {
  users: User[];
  onEdit: (user: User) => void;
  onDelete: (id: string) => void;
}

export function UserTable({ users, onEdit, onDelete }: Props) {
  if (users.length === 0) {
    return (
      <div className="card bg-base-100 shadow">
        <div className="card-body items-center text-center text-base-content/60">
          <p>No users yet. Create one above.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="card bg-base-100 shadow overflow-x-auto">
      <table className="table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <tr key={user.id} className="hover">
              <td>{user.id}</td>
              <td>{user.name}</td>
              <td>{user.email}</td>
              <td>
                <div className="flex gap-1">
                  <button className="btn btn-ghost btn-xs" onClick={() => onEdit(user)}>
                    Edit
                  </button>
                  <button className="btn btn-ghost btn-xs text-error" onClick={() => onDelete(user.id)}>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
