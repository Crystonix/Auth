// lib/auth/logout.ts
import { user } from '$lib/stores/user.svelte';

export async function logout() {
	try {
		await fetch('http://localhost/api/auth/logout', {
			method: 'POST',
			credentials: 'include'
		});
	} finally {
		// Always reset client state, even if request fails
		user.reset();
	}
}
