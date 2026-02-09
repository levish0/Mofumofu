export interface CreatePostRequest {
	title: string;
	content: string;
	slug: string;
	summary?: string | null;
	hashtags?: string[] | null;
}

export interface CreatePostResponse {
	post_id: string;
}

export interface GetPostByHandleAndSlugRequest {
	handle: string;
	slug: string;
}

export interface GetPostByUuidRequest {
	post_id: string;
}

export interface PostAuthor {
	handle: string;
	name: string;
	profile_image?: string | null;
}

export interface TocItem {
	level: number;
	text: string;
	id: string;
}

export interface PostInfoResponse {
	id: string;
	title: string;
	rendered: string;
	toc_items: TocItem[];
	author: PostAuthor;
	created_at: string;
	like_count: number;
	comment_count: number;
	view_count: number;
	slug: string;
	summary?: string | null;
	thumbnail_image?: string | null;
	updated_at?: string | null;
	tags: string[];
}

export type PostSortOrder = 'latest' | 'popular' | 'oldest';

export interface GetPostsRequest {
	page?: number;
	page_size?: number;
	sort?: PostSortOrder;
}

export interface SearchPostsRequest {
	query?: string | null;
	hashtags?: string[] | null;
	date_from?: string | null;
	date_to?: string | null;
	min_likes?: number | null;
	user_handle?: string | null;
	sort?: PostSortOrder | null;
	page?: number | null;
	page_size?: number | null;
}

export interface GetUserPostsRequest {
	user_handle: string;
}

export interface UserPostsResponse {
	posts: PostListItem[];
}

export interface PostListItem {
	id: string;
	title: string;
	summary?: string | null;
	thumbnail_image?: string | null;
	user_handle: string;
	user_name: string;
	user_avatar?: string | null;
	created_at: string;
	like_count: number;
	comment_count: number;
	view_count: number;
	slug: string;
	hashtags: string[];
}

export interface GetPostsResponse {
	posts: PostListItem[];
	current_page: number;
	page_size: number;
	has_more: boolean;
	total_count: number;
}

export interface ThumbnailUploadRequest {
	post_id: string;
	file: File;
}

export interface ThumbnailUploadResponse {
	public_url: string;
}

export interface ImageUploadRequest {
	file: File;
}

export interface PostImageUploadResponse {
	public_url: string;
}

export interface UpdatePostRequest {
	post_id: string;
	title?: string | null;
	content?: string | null;
	summary?: string | null;
	hashtags?: string[] | null;
	new_slug?: string | null;
}

export interface DeletePostRequest {
	slug: string;
}

export interface GetPostForEditRequest {
	slug: string;
}

export interface PostEditInfoResponse {
	id: string;
	title: string;
	content: string;
	created_at: string;
	slug: string;
	tags: string[];
	summary?: string | null;
	thumbnail_image?: string | null;
	updated_at?: string | null;
}
