import type { UserInfoResponse } from '../user/types';

export interface CreateFollowRequest {
	followee_handle: string;
}

export interface DeleteFollowRequest {
	followee_handle: string;
}

export interface GetFollowCountRequest {
	user_handle: string;
}

export interface FollowCountResponse {
	count: number;
}

export interface CheckFollowStatusRequest {
	handle: string;
}

export interface FollowStatusResponse {
	is_following: boolean;
}

export interface FollowListResponse {
	has_more: boolean;
	page: number;
	per_page: number;
	total_count: number;
	users: UserInfoResponse[];
}
