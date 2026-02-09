import { privateApi } from '../private';
import type { CreateLikeRequest, DeleteLikeRequest, CheckLikeStatusRequest, LikeStatusResponse } from './types';

export async function createLike(request: CreateLikeRequest) {
	try {
		await privateApi.post('v0/like', {
			json: request
		});
	} catch (error) {
		console.error('Failed to create like:', error);
		throw error;
	}
}

export async function deleteLike(request: DeleteLikeRequest) {
	try {
		await privateApi.delete('v0/like', {
			json: request
		});
	} catch (error) {
		console.error('Failed to delete like:', error);
		throw error;
	}
}

export async function checkLikeStatus(request: CheckLikeStatusRequest): Promise<LikeStatusResponse> {
	try {
		const response = await privateApi
			.post('v0/like/status', {
				json: request
			})
			.json<LikeStatusResponse>();
		return response;
	} catch (error) {
		console.error('Failed to check like status:', error);
		throw error;
	}
}
