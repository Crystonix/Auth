// src/routes/dashboard/+layout.ts
import { requireAuth } from '$lib/auth/ssr';
import { Roles } from '$lib/stores/user.svelte';
import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ fetch }) => {
  await requireAuth(fetch, Roles.USER);
};
