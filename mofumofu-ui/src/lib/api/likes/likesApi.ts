import type { KyInstance } from 'ky';
import { Api } from '../api';
import type { LikeStatusResponse, LikeTargetType } from '../types';

export async function createLike(
	targetId: string,
	targetType: LikeTargetType,
	api: KyInstance = Api
): Promise<LikeStatusResponse> {
	return api
		.post('v0/likes', { json: { target_id: targetId, target_type: targetType } })
		.json<LikeStatusResponse>();
}

export async function deleteLike(
	targetId: string,
	targetType: LikeTargetType,
	api: KyInstance = Api
): Promise<LikeStatusResponse> {
	return api
		.delete('v0/likes', { json: { target_id: targetId, target_type: targetType } })
		.json<LikeStatusResponse>();
}

export async function checkLikeStatus(
	targetId: string,
	targetType: LikeTargetType,
	api: KyInstance = Api
): Promise<LikeStatusResponse> {
	return api
		.get('v0/likes/status', {
			searchParams: { target_id: targetId, target_type: targetType }
		})
		.json<LikeStatusResponse>();
}
