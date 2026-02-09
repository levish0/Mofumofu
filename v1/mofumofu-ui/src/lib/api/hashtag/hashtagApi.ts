import { publicApi } from '../public';
import type { TrendingHashtagsRequest, TrendingHashtagsResponse } from './types';

export async function getTrendingHashtags(request: TrendingHashtagsRequest = {}): Promise<TrendingHashtagsResponse> {
	try {
		const response = await publicApi.post('v0/hashtag/trending', { json: request });
		return response.json<TrendingHashtagsResponse>();
	} catch (error) {
		console.error('Failed to get trending hashtags:', error);
		throw error;
	}
}
