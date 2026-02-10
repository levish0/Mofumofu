import type { KyInstance } from 'ky';
import { Api } from '../api';
import type {
	LoginRequest,
	LoginResult,
	UserResponse,
	CreateUserRequest,
	CreateUserResponse,
	VerifyEmailRequest,
	ForgotPasswordRequest,
	ResetPasswordRequest,
	ChangePasswordRequest,
	ChangeEmailRequest,
	ConfirmEmailChangeRequest
} from '../types';

export async function login(data: LoginRequest, api: KyInstance = Api): Promise<LoginResult> {
	const response = await api.post('v0/auth/login', { json: data });

	if (response.status === 202) {
		const body = await response.json<{ temp_token: string }>();
		return { kind: 'totp_required', temp_token: body.temp_token };
	}

	return { kind: 'success' };
}

export async function logout(api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/logout');
}

export async function getMe(api: KyInstance = Api): Promise<UserResponse | null> {
	try {
		return await api.get('v0/user/me').json<UserResponse>();
	} catch {
		return null;
	}
}

export async function register(
	data: CreateUserRequest,
	api: KyInstance = Api
): Promise<CreateUserResponse> {
	return api.post('v0/users', { json: data }).json<CreateUserResponse>();
}

export async function verifyEmail(token: string, api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/verify-email', { json: { token } satisfies VerifyEmailRequest });
}

export async function resendVerificationEmail(api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/resend-verification-email');
}

export async function forgotPassword(email: string, api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/forgot-password', {
		json: { email } satisfies ForgotPasswordRequest
	});
}

export async function resetPassword(
	data: ResetPasswordRequest,
	api: KyInstance = Api
): Promise<void> {
	await api.post('v0/auth/reset-password', { json: data });
}

export async function changePassword(
	data: ChangePasswordRequest,
	api: KyInstance = Api
): Promise<void> {
	await api.post('v0/auth/change-password', { json: data });
}

export async function changeEmail(data: ChangeEmailRequest, api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/change-email', { json: data });
}

export async function confirmEmailChange(token: string, api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/confirm-email-change', {
		json: { token } satisfies ConfirmEmailChangeRequest
	});
}
