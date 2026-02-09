export interface TrendingHashtagsRequest {
	days?: number | null;
	limit?: number | null;
}

export interface TrendingHashtagsResponse {
	hashtags: string[];
}
