// src/lib/auth/ssr.ts
import { user, Roles } from '$lib/stores/user.svelte';
import { redirect } from '@sveltejs/kit';

export async function requireAuth(fetch: typeof window.fetch, role?: Roles) {
  const res = await fetch('localhost/api/auth/me', { credentials: 'include' });

  if (!res.ok) {
    user.reset();
    throw redirect(303, '/login');
  }

  const data = await res.json();
  user.setAuthenticated(data.role as Roles);

  if (role && data.role !== role) {
    throw redirect(303, '/403');
  }
}
