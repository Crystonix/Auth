// src/routes/admin/+layout.server.ts
import { Roles } from '$lib/auth/user';
import { redirect, type Load } from '@sveltejs/kit';

export const load: Load = async ({ parent }) => {
  const { user } = await parent();

	if (!user?.authenticated) {
    throw redirect(302, '/');
  }

  if (user.role !== Roles.ADMIN) {
    throw redirect(302, '/dashboard');
  }

  return { user };
};
