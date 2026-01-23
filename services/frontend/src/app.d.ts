// See https://svelte.dev/docs/kit/types#app.d.ts

import type { AuthUser } from "$lib/stores";

// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: AuthUser | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
