import type { KyInstance } from 'ky';
import type { UserResponse } from '$lib/api/types';

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			api: KyInstance;
		}
		interface PageData {
			user: UserResponse | null;
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
