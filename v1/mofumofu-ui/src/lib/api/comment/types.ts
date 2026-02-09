export interface CreateCommentRequest {
	content: string;
	post_id: string;
	parent_id?: string | null;
}

export interface CreateCommentResponse {
	comment_id: string;
}

export interface UpdateCommentRequest {
	comment_id: string;
	content: string;
}

export interface DeleteCommentRequest {
	comment_id: string;
}

export interface GetCommentByIdRequest {
	comment_id: string;
}

export interface GetCommentsRequest {
	post_id: string;
	page?: number;
	per_page?: number;
	sort?: 'latest' | 'oldest' | 'popular';
}

export interface GetRepliesRequest {
	parent_comment_id: string;
	page?: number;
	per_page?: number;
	sort?: 'latest' | 'oldest' | 'popular';
}

export interface CommentInfo {
	id: string;
	post_id: string;
	content?: string | null;
	parent_id?: string | null;
	user_id?: string | null;
	user_handle?: string | null;
	user_name?: string | null;
	user_profile_image?: string | null;
	like_count: number;
	reply_count: number;
	is_deleted: boolean;
	created_at: string;
	updated_at?: string | null;
}

export interface GetCommentsResponse {
	comments: CommentInfo[];
	total_count: number;
	page: number;
	per_page: number;
	has_next: boolean;
}

export interface GetRepliesResponse {
	replies: CommentInfo[];
	total_count: number;
	page: number;
	per_page: number;
	has_next: boolean;
}

export interface CreateCommentLikeRequest {
	comment_id: string;
}

export interface DeleteCommentLikeRequest {
	comment_id: string;
}

export interface CheckCommentLikeStatusRequest {
	comment_id: string;
}
