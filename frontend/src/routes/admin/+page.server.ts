// src/routes/admin/+page.server.ts
import { requireUser } from '$lib/auth/server';
import { Roles } from '$lib/auth/user';
import type { ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = ({ locals }) => {
    const user = requireUser(locals.user, { roles: [Roles.ADMIN], redirectTo: '/login' });
    return { user };
};
