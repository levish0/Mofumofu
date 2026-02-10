import type { KyInstance } from 'ky';
import { Api } from '../api';
import type {
	OAuthUrlResponse,
	OAuthLoginResult,
	OAuthPendingSignupResponse,
	CompleteSignupRequest,
	OAuthConnectionListResponse,
	OAuthProvider
} from '../types';

export async function getGoogleAuthUrl(api: KyInstance = Api): Promise<OAuthUrlResponse> {
	return api.get('v0/auth/oauth/google/authorize').json<OAuthUrlResponse>();
}

export async function getGithubAuthUrl(api: KyInstance = Api): Promise<OAuthUrlResponse> {
	return api.get('v0/auth/oauth/github/authorize').json<OAuthUrlResponse>();
}

export async function loginWithGoogle(
	code: string,
	state: string,
	api: KyInstance = Api
): Promise<OAuthLoginResult> {
	const response = await api.post('v0/auth/oauth/google/login', { json: { code, state } });

	if (response.status === 200) {
		const data = await response.json<OAuthPendingSignupResponse>();
		return { kind: 'new_user', data };
	}

	return { kind: 'existing_user' };
}

export async function loginWithGithub(
	code: string,
	state: string,
	api: KyInstance = Api
): Promise<OAuthLoginResult> {
	const response = await api.post('v0/auth/oauth/github/login', { json: { code, state } });

	if (response.status === 200) {
		const data = await response.json<OAuthPendingSignupResponse>();
		return { kind: 'new_user', data };
	}

	return { kind: 'existing_user' };
}

export async function completeSignup(
	data: CompleteSignupRequest,
	api: KyInstance = Api
): Promise<void> {
	await api.post('v0/auth/complete-signup', { json: data });
}

export async function linkGoogle(
	code: string,
	state: string,
	api: KyInstance = Api
): Promise<void> {
	await api.post('v0/auth/oauth/google/link', { json: { code, state } });
}

export async function linkGithub(
	code: string,
	state: string,
	api: KyInstance = Api
): Promise<void> {
	await api.post('v0/auth/oauth/github/link', { json: { code, state } });
}

export async function unlinkOAuth(provider: OAuthProvider, api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/oauth/connections/unlink', { json: { provider } });
}

export async function listOAuthConnections(
	api: KyInstance = Api
): Promise<OAuthConnectionListResponse> {
	return api.get('v0/auth/oauth/connections').json<OAuthConnectionListResponse>();
}
