import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, cookies }) => {
	const state = url.searchParams.get('state');
	const code = url.searchParams.get('code');
	const errorParam = url.searchParams.get('error');

	// If there's an OAuth error, let the client handle it
	if (errorParam) {
		return {
			error: `OAuth error: ${errorParam}`
		};
	}

	// Code is required for OAuth flow
	if (!code) {
		return {
			error: 'Authorization code not found'
		};
	}

	// Verify state parameter for CSRF protection (for link requests)
	const storedState = cookies.get('oauth_link_state');
	if (!state || !storedState || state !== storedState) {
		console.error('Link state verification failed', { state, storedState });
		throw error(400, 'Invalid state parameter - potential CSRF attack');
	}

	// Clean up cookies after verification
	cookies.delete('oauth_link_state', { path: '/' });

	return {
		code
	};
};
