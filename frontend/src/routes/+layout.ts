// src/routes/+layout.ts
import { user } from '$lib/stores/user.svelte';
import type { LayoutData } from './$types';

export const load = ({ data }: { data: LayoutData }) => {
    if (data?.user) {
        user.setUser(data.user);
    } else {
        user.reset();
    }
};
