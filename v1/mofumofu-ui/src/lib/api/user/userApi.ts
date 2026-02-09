// src/lib/api/user/userApi.ts

import { privateApi } from '../private';
import { publicApi } from '../public';
import type {
	UserInfoResponse,
	GetUserProfileRequest,
	UpdateProfileRequest,
	HandleCheckResponse,
	ImageUploadResponse
} from './types';

export async function getMyProfile(): Promise<UserInfoResponse> {
	try {
		console.log('Fetching user profile...');
		const response = await privateApi.get('v0/user/my_profile');
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function getUserProfile(handle: string): Promise<UserInfoResponse> {
	try {
		const payload: GetUserProfileRequest = { handle };
		const response = await publicApi.post('v0/user/profile', { json: payload });
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function updateProfile(data: UpdateProfileRequest): Promise<UserInfoResponse> {
	try {
		const response = await privateApi.put('v0/user/profile', { json: data });
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function uploadAvatar(file: File): Promise<ImageUploadResponse> {
	try {
		const formData = new FormData();
		formData.append('file', file);

		const response = await privateApi.post('v0/user/profile/avatar', {
			body: formData,
			headers: {
				'Content-Type': undefined // Remove default Content-Type header
			}
		});
		return await response.json<ImageUploadResponse>();
	} catch (error) {
		console.error('Avatar upload failed:', error);
		throw error;
	}
}

export async function uploadBanner(file: File): Promise<ImageUploadResponse> {
	try {
		const formData = new FormData();
		formData.append('file', file);

		const response = await privateApi.post('v0/user/profile/banner', {
			body: formData,
			headers: {
				'Content-Type': undefined // Remove default Content-Type header
			}
		});
		return await response.json<ImageUploadResponse>();
	} catch (error) {
		console.error('Banner upload failed:', error);
		throw error;
	}
}

export async function checkHandleAvailability(handle: string): Promise<HandleCheckResponse> {
	try {
		const payload: GetUserProfileRequest = { handle };
		const response = await publicApi.post('v0/user/check-handle', { json: payload });
		return await response.json<HandleCheckResponse>();
	} catch (error) {
		console.error('Handle check failed:', error);
		throw error;
	}
}
