import type { KyInstance } from 'ky';
import { Api } from '../api';
import type {
	UserResponse,
	PublicUserProfile,
	UpdateMyProfileRequest,
	CheckHandleAvailableResponse,
	SearchUsersResponse,
	UploadUserImageResponse
} from '../types';

export async function getMyProfile(api: KyInstance = Api): Promise<UserResponse> {
	return api.get('v0/user/me').json<UserResponse>();
}

export async function getUserProfile(
	handle: string,
	api: KyInstance = Api
): Promise<PublicUserProfile> {
	return api.get('v0/users/profile', { searchParams: { handle } }).json<PublicUserProfile>();
}

export async function getUserProfileById(
	userId: string,
	api: KyInstance = Api
): Promise<PublicUserProfile> {
	return api
		.get('v0/users/profile/id', { searchParams: { user_id: userId } })
		.json<PublicUserProfile>();
}

export async function updateMyProfile(
	data: UpdateMyProfileRequest,
	api: KyInstance = Api
): Promise<UserResponse> {
	return api.patch('v0/user/me', { json: data }).json<UserResponse>();
}

export async function checkHandleAvailable(
	handle: string,
	api: KyInstance = Api
): Promise<CheckHandleAvailableResponse> {
	return api
		.get(`v0/users/handle/${encodeURIComponent(handle)}/available`)
		.json<CheckHandleAvailableResponse>();
}

export async function searchUsers(
	params: { query?: string; page: number; page_size: number },
	api: KyInstance = Api
): Promise<SearchUsersResponse> {
	return api.get('v0/search/users', { searchParams: params }).json<SearchUsersResponse>();
}

export async function uploadProfileImage(
	file: File,
	api: KyInstance = Api
): Promise<UploadUserImageResponse> {
	const formData = new FormData();
	formData.append('file', file);
	return api
		.post('v0/user/me/profile-image', {
			body: formData,
			headers: { 'Content-Type': undefined! }
		})
		.json<UploadUserImageResponse>();
}

export async function deleteProfileImage(api: KyInstance = Api): Promise<void> {
	await api.delete('v0/user/me/profile-image');
}

export async function uploadBannerImage(
	file: File,
	api: KyInstance = Api
): Promise<UploadUserImageResponse> {
	const formData = new FormData();
	formData.append('file', file);
	return api
		.post('v0/user/me/banner-image', {
			body: formData,
			headers: { 'Content-Type': undefined! }
		})
		.json<UploadUserImageResponse>();
}

export async function deleteBannerImage(api: KyInstance = Api): Promise<void> {
	await api.delete('v0/user/me/banner-image');
}
