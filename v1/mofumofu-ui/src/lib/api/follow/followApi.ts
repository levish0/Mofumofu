import { privateApi } from '../private';
import { publicApi } from '../public';
import type {
	CreateFollowRequest,
	DeleteFollowRequest,
	GetFollowCountRequest,
	FollowCountResponse,
	CheckFollowStatusRequest,
	FollowStatusResponse
} from './types';

export async function createFollow(request: CreateFollowRequest) {
	try {
		await privateApi.post('v0/follow', {
			json: request
		});
	} catch (error) {
		console.error('Failed to create follow:', error);
		throw error;
	}
}

export async function deleteFollow(request: DeleteFollowRequest) {
	try {
		await privateApi.delete('v0/follow', {
			json: request
		});
	} catch (error) {
		console.error('Failed to delete follow:', error);
		throw error;
	}
}

export async function getFollowerCount(request: GetFollowCountRequest): Promise<FollowCountResponse> {
	try {
		const response = await publicApi
			.post('v0/follow/follower-count', {
				json: request
			})
			.json<FollowCountResponse>();
		return response;
	} catch (error) {
		console.error('Failed to get follower count:', error);
		throw error;
	}
}

export async function getFollowingCount(request: GetFollowCountRequest): Promise<FollowCountResponse> {
	try {
		const response = await publicApi
			.post('v0/follow/following-count', {
				json: request
			})
			.json<FollowCountResponse>();
		return response;
	} catch (error) {
		console.error('Failed to get following count:', error);
		throw error;
	}
}

export async function checkFollowStatus(request: CheckFollowStatusRequest): Promise<FollowStatusResponse> {
	try {
		const response = await privateApi
			.post('v0/follow/status', {
				json: request
			})
			.json<FollowStatusResponse>();
		return response;
	} catch (error) {
		console.error('Failed to check follow status:', error);
		throw error;
	}
}
