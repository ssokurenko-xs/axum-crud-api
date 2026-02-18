import { useState, useEffect, type FormEvent } from 'react';
import type { User, UserPayload } from './api';

interface Props {
  editing: User | null;
  onSubmit: (payload: UserPayload) => Promise<void>;
  onCancel: () => void;
}

export function UserForm({ editing, onSubmit, onCancel }: Props) {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (editing) {
      setName(editing.name);
      setEmail(editing.email);
    } else {
      setName('');
      setEmail('');
    }
    setError(null);
  }, [editing]);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !email.trim()) return;
    setSubmitting(true);
    setError(null);
    try {
      await onSubmit({ name: name.trim(), email: email.trim() });
      if (!editing) {
        setName('');
        setEmail('');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Something went wrong');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="card bg-base-100 shadow mb-6">
      <div className="card-body">
        <h2 className="card-title">{editing ? 'Edit User' : 'New User'}</h2>
        {error && (
          <div role="alert" className="alert alert-error alert-sm">
            <span>{error}</span>
          </div>
        )}
        <form onSubmit={handleSubmit} className="flex flex-col gap-3">
          <label className="floating-label">
            <span>Name</span>
            <input
              type="text"
              className="input input-bordered w-full"
              placeholder="Name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </label>
          <label className="floating-label">
            <span>Email</span>
            <input
              type="email"
              className="input input-bordered w-full"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </label>
          <div className="card-actions justify-end mt-2">
            {editing && (
              <button type="button" className="btn btn-ghost" onClick={onCancel}>
                Cancel
              </button>
            )}
            <button type="submit" className="btn btn-primary" disabled={submitting}>
              {submitting && <span className="loading loading-spinner loading-xs"></span>}
              {editing ? 'Update' : 'Create'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
