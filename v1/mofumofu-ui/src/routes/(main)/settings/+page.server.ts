import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { PUBLIC_GOOGLE_CLIENT_ID, PUBLIC_GITHUB_CLIENT_ID, PUBLIC_APP_URL } from '$env/static/public';

export const actions: Actions = {
	linkGoogle: async ({ cookies }) => {
		// Generate secure random state for Google link
		const state = `link_${crypto.randomUUID()}`;

		// Store state in HttpOnly cookie (expires in 10 minutes)
		cookies.set('oauth_link_state', state, {
			httpOnly: true,
			secure: true,
			sameSite: 'lax',
			path: '/',
			maxAge: 60 * 10 // 10 minutes
		});

		// Build Google OAuth URL with state (uses separate link callback)
		const params = new URLSearchParams({
			client_id: PUBLIC_GOOGLE_CLIENT_ID,
			redirect_uri: `${PUBLIC_APP_URL}/account/oauth/link/google`,
			response_type: 'code',
			scope: 'openid email profile',
			access_type: 'offline',
			prompt: 'consent',
			state
		});

		const authUrl = `https://accounts.google.com/o/oauth2/v2/auth?${params.toString()}`;
		throw redirect(302, authUrl);
	},

	linkGithub: async ({ cookies }) => {
		// Generate secure random state for GitHub link
		const state = `link_${crypto.randomUUID()}`;

		// Store state in HttpOnly cookie (expires in 10 minutes)
		cookies.set('oauth_link_state', state, {
			httpOnly: true,
			secure: true,
			sameSite: 'lax',
			path: '/',
			maxAge: 60 * 10 // 10 minutes
		});

		// Build GitHub OAuth URL with state (uses main callback with state detection)
		const params = new URLSearchParams({
			client_id: PUBLIC_GITHUB_CLIENT_ID,
			redirect_uri: `${PUBLIC_APP_URL}/account/oauth/callback/github`,
			scope: 'user:email',
			state
		});

		const authUrl = `https://github.com/login/oauth/authorize?${params.toString()}`;
		throw redirect(302, authUrl);
	}
};
