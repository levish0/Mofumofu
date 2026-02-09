// src/lib/oauth/config.ts

import { PUBLIC_GOOGLE_CLIENT_ID, PUBLIC_GITHUB_CLIENT_ID, PUBLIC_APP_URL } from '$env/static/public';

export const OAUTH_CONFIG = {
	google: {
		clientId: PUBLIC_GOOGLE_CLIENT_ID,
		redirectUri: `${PUBLIC_APP_URL}/account/oauth/callback/google`,
		linkRedirectUri: `${PUBLIC_APP_URL}/account/oauth/link/google`,
		scope: 'openid email profile',
		responseType: 'code',
		authUrl: 'https://accounts.google.com/o/oauth2/v2/auth'
	},
	github: {
		clientId: PUBLIC_GITHUB_CLIENT_ID,
		redirectUri: `${PUBLIC_APP_URL}/account/oauth/callback/github`,
		scope: 'user:email',
		authUrl: 'https://github.com/login/oauth/authorize'
	}
};

// OAuth URL generation functions have been moved to server actions for CSRF protection:
// - getGoogleOAuthUrl() → /account/signin/+page.server.ts googleOAuth action
// - getGitHubOAuthUrl() → /account/signin/+page.server.ts githubOAuth action
// - getGoogleOAuthLinkUrl() → /settings/+page.server.ts linkGoogle action
// - getGitHubOAuthLinkUrl() → /settings/+page.server.ts linkGithub action
