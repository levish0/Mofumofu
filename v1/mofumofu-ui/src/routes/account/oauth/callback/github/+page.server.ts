import { error, redirect } from '@sveltejs/kit';
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

	// Check if this is a link request (state starts with 'link_')
	const isLinkRequest = state?.startsWith('link_') ?? false;

	if (isLinkRequest) {
		// Verify link state parameter for CSRF protection
		const storedLinkState = cookies.get('oauth_link_state');
		if (!state || !storedLinkState || state !== storedLinkState) {
			console.error('Link state verification failed', { state, storedLinkState });
			throw error(400, 'Invalid link state parameter - potential CSRF attack');
		}

		// Clean up link state cookie after verification
		cookies.delete('oauth_link_state', { path: '/' });

		return {
			code,
			handle: null,
			isLinkRequest: true
		};
	}

	// Verify state parameter for CSRF protection
	const storedState = cookies.get('oauth_state');
	if (!state || !storedState || state !== storedState) {
		console.error('State verification failed', { state, storedState });
		throw error(400, 'Invalid state parameter - potential CSRF attack');
	}

	// Get handle if this is a signup flow
	const handle = cookies.get('oauth_handle');

	// Clean up cookies after verification
	cookies.delete('oauth_state', { path: '/' });
	if (handle) {
		cookies.delete('oauth_handle', { path: '/' });
	}

	return {
		code,
		handle: handle || null,
		isLinkRequest: false
	};
};
