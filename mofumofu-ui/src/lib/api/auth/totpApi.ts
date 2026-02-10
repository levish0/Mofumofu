import type { KyInstance } from 'ky';
import { Api } from '../api';
import type {
	TotpSetupResponse,
	TotpEnableResponse,
	TotpStatusResponse,
	TotpBackupCodesResponse
} from '../types';

export async function totpSetup(api: KyInstance = Api): Promise<TotpSetupResponse> {
	return api.post('v0/auth/totp/setup').json<TotpSetupResponse>();
}

export async function totpEnable(code: string, api: KyInstance = Api): Promise<TotpEnableResponse> {
	return api.post('v0/auth/totp/enable', { json: { code } }).json<TotpEnableResponse>();
}

export async function totpDisable(code: string, api: KyInstance = Api): Promise<void> {
	await api.post('v0/auth/totp/disable', { json: { code } });
}

export async function totpStatus(api: KyInstance = Api): Promise<TotpStatusResponse> {
	return api.get('v0/auth/totp/status').json<TotpStatusResponse>();
}

export async function totpVerify(
	code: string,
	tempToken: string,
	api: KyInstance = Api
): Promise<void> {
	await api.post('v0/auth/totp/verify', { json: { code, temp_token: tempToken } });
}

export async function totpRegenerateBackupCodes(
	code: string,
	api: KyInstance = Api
): Promise<TotpBackupCodesResponse> {
	return api
		.post('v0/auth/totp/backup-codes/regenerate', { json: { code } })
		.json<TotpBackupCodesResponse>();
}
