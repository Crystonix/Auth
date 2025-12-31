// src/routes/+layout.ts
import { user } from '$lib/stores/user.svelte';
import type { AuthUser } from '$lib/stores/user.svelte';
import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ fetch }) => {
    try {
        // Try to fetch current session (will succeed if session_id cookie exists)
        const res = await fetch('/api/auth/me', { credentials: 'include' });

        if (!res.ok) {
            // No session or expired → reset store
            user.reset();
            return {};
        }

        const data: AuthUser = await res.json();

        // Populate user store
        user.setUser(data);

        return {
            user: {
                id: data.id,
                username: data.username,
                avatar: data.avatar ?? null,
                role: data.role
            }
        };
    } catch (err) {
        console.error('SSR auth load error:', err);
        user.reset();
        return {};
    }
};
