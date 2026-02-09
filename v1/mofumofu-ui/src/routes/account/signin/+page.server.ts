import { redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { PUBLIC_GOOGLE_CLIENT_ID, PUBLIC_GITHUB_CLIENT_ID, PUBLIC_APP_URL } from '$env/static/public';

export const load: PageServerLoad = async () => {
	return {};
};

export const actions: Actions = {
	googleOAuth: async ({ cookies }) => {
		// Generate secure random state
		const state = crypto.randomUUID();

		// Store state in HttpOnly cookie (expires in 10 minutes)
		cookies.set('oauth_state', state, {
			httpOnly: true,
			secure: true,
			sameSite: 'lax',
			path: '/',
			maxAge: 60 * 10 // 10 minutes
		});

		// Build Google OAuth URL with state
		const params = new URLSearchParams({
			client_id: PUBLIC_GOOGLE_CLIENT_ID,
			redirect_uri: `${PUBLIC_APP_URL}/account/oauth/callback/google`,
			response_type: 'code',
			scope: 'openid email profile',
			access_type: 'offline',
			prompt: 'consent',
			state
		});

		const authUrl = `https://accounts.google.com/o/oauth2/v2/auth?${params.toString()}`;
		throw redirect(302, authUrl);
	},

	githubOAuth: async ({ cookies }) => {
		// Generate secure random state
		const state = crypto.randomUUID();

		// Store state in HttpOnly cookie (expires in 10 minutes)
		cookies.set('oauth_state', state, {
			httpOnly: true,
			secure: true,
			sameSite: 'lax',
			path: '/',
			maxAge: 60 * 10 // 10 minutes
		});

		// Build GitHub OAuth URL with state
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
