// src/lib/auth/server.ts
import { error, redirect } from '@sveltejs/kit';
import type { AuthUser } from './user';

export function requireUser(
    user: AuthUser | null,
    options?: { roles?: AuthUser['role'][]; redirectTo?: string }
): AuthUser {
    if (!user) {
        if (options?.redirectTo) throw redirect(307, options.redirectTo);
        throw error(401, 'Not authenticated');
    }

    if (options?.roles && !options.roles.includes(user.role)) {
        throw error(403, 'Forbidden');
    }

    return user;
}
