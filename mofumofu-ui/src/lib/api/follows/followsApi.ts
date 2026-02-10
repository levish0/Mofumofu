import type { KyInstance } from 'ky';
import { Api } from '../api';
import type { FollowStatusResponse, FollowListResponse, CursorDirection } from '../types';

export async function createFollow(
	followeeId: string,
	api: KyInstance = Api
): Promise<FollowStatusResponse> {
	return api.post('v0/follows', { json: { followee_id: followeeId } }).json<FollowStatusResponse>();
}

export async function deleteFollow(
	followeeId: string,
	api: KyInstance = Api
): Promise<FollowStatusResponse> {
	return api
		.delete('v0/follows', { json: { followee_id: followeeId } })
		.json<FollowStatusResponse>();
}

export async function checkFollowStatus(
	followeeId: string,
	api: KyInstance = Api
): Promise<FollowStatusResponse> {
	return api
		.get('v0/follows/status', { searchParams: { followee_id: followeeId } })
		.json<FollowStatusResponse>();
}

export async function getFollowers(
	params: {
		user_id: string;
		limit: number;
		cursor_id?: string;
		cursor_direction?: CursorDirection;
	},
	api: KyInstance = Api
): Promise<FollowListResponse> {
	return api.get('v0/follows/followers', { searchParams: params }).json<FollowListResponse>();
}

export async function getFollowing(
	params: {
		user_id: string;
		limit: number;
		cursor_id?: string;
		cursor_direction?: CursorDirection;
	},
	api: KyInstance = Api
): Promise<FollowListResponse> {
	return api.get('v0/follows/following', { searchParams: params }).json<FollowListResponse>();
}
