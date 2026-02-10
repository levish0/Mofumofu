import type { KyInstance } from 'ky';
import { Api } from '../api';
import type {
	PostResponse,
	PostListResponse,
	SearchPostsResponse,
	CreatePostRequest,
	UpdatePostRequest,
	UploadPostImageResponse,
	PostSortField,
	SortOrder,
	CursorDirection
} from '../types';

export async function searchPosts(
	params: {
		query?: string;
		user_id?: string;
		published_at_after?: string;
		page: number;
		page_size: number;
		sort_by?: PostSortField;
		sort_order?: SortOrder;
	},
	api: KyInstance = Api
): Promise<SearchPostsResponse> {
	return api.get('v0/search/posts', { searchParams: params }).json<SearchPostsResponse>();
}

export async function getPostBySlug(
	handle: string,
	slug: string,
	api: KyInstance = Api
): Promise<PostResponse> {
	return api.get('v0/posts/by-slug', { searchParams: { handle, slug } }).json<PostResponse>();
}

export async function getPosts(
	params: {
		limit: number;
		cursor_id?: string;
		cursor_direction?: CursorDirection;
		user_id?: string;
		published_only?: boolean;
	},
	api: KyInstance = Api
): Promise<PostListResponse> {
	return api.get('v0/posts', { searchParams: params }).json<PostListResponse>();
}

export async function getPost(postId: string, api: KyInstance = Api): Promise<PostResponse> {
	return api.get(`v0/posts/${encodeURIComponent(postId)}`).json<PostResponse>();
}

export async function createPost(
	data: CreatePostRequest,
	api: KyInstance = Api
): Promise<PostResponse> {
	return api.post('v0/posts', { json: data }).json<PostResponse>();
}

export async function updatePost(
	postId: string,
	data: UpdatePostRequest,
	api: KyInstance = Api
): Promise<PostResponse> {
	return api
		.patch(`v0/posts/${encodeURIComponent(postId)}`, { json: data })
		.json<PostResponse>();
}

export async function deletePost(postId: string, api: KyInstance = Api): Promise<void> {
	await api.delete(`v0/posts/${encodeURIComponent(postId)}`);
}

export async function incrementView(postId: string, api: KyInstance = Api): Promise<void> {
	await api.post(`v0/posts/${encodeURIComponent(postId)}/view`);
}

export async function uploadPostImage(
	file: File,
	api: KyInstance = Api
): Promise<UploadPostImageResponse> {
	const formData = new FormData();
	formData.append('file', file);
	const response = await api.post('v0/posts/images', {
		body: formData,
		headers: { 'Content-Type': undefined! }
	});
	return response.json<UploadPostImageResponse>();
}
