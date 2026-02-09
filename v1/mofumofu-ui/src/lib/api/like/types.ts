export interface CreateLikeRequest {
	post_id: string;
}

export interface DeleteLikeRequest {
	post_id: string;
}

export interface CheckLikeStatusRequest {
	post_id: string;
}

export interface LikeStatusResponse {
	is_liked: boolean;
}
