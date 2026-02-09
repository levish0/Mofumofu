import { redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { PUBLIC_GITHUB_CLIENT_ID, PUBLIC_APP_URL } from '$env/static/public';

export const load: PageServerLoad = async () => {
	return {};
};

export const actions: Actions = {
	proceedWithGitHub: async ({ cookies, request }) => {
		const data = await request.formData();
		const handle = data.get('handle') as string;

		if (!handle) {
			return { error: 'Handle is required' };
		}

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

		// Store handle for signup flow
		cookies.set('oauth_handle', handle, {
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
