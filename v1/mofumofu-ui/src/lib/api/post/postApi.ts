import { privateApi } from '../private';
import { publicApi } from '../public';
import type {
	CreatePostRequest,
	CreatePostResponse,
	GetPostByHandleAndSlugRequest,
	GetPostByUuidRequest,
	PostInfoResponse,
	GetPostsRequest,
	SearchPostsRequest,
	GetPostsResponse,
	ThumbnailUploadRequest,
	GetUserPostsRequest,
	UserPostsResponse,
	UpdatePostRequest,
	DeletePostRequest,
	GetPostForEditRequest,
	PostEditInfoResponse,
	ImageUploadRequest,
	PostImageUploadResponse
} from './types';

export async function createPost(postData: CreatePostRequest): Promise<CreatePostResponse> {
	try {
		const response = await privateApi.post('v0/post', { json: postData });
		return response.json<CreatePostResponse>();
	} catch (error) {
		console.error('Failed to create post:', error);
		throw error;
	}
}

export async function getPostByHandleAndSlug(request: GetPostByHandleAndSlugRequest): Promise<PostInfoResponse> {
	try {
		const response = await publicApi.post('v0/post/get_by_handle_and_slug', { json: request });
		return response.json<PostInfoResponse>();
	} catch (error) {
		console.error('Failed to get post:', error);
		throw error;
	}
}

export async function getPostByUuid(request: GetPostByUuidRequest): Promise<PostInfoResponse> {
	try {
		const response = await publicApi.post('v0/post/get', { json: request });
		return response.json<PostInfoResponse>();
	} catch (error) {
		console.error('Failed to get post:', error);
		throw error;
	}
}

export async function getPosts(request: GetPostsRequest = {}): Promise<GetPostsResponse> {
	try {
		const response = await publicApi.post('v0/posts', { json: request });
		return response.json<GetPostsResponse>();
	} catch (error) {
		console.error('Failed to get posts:', error);
		throw error;
	}
}

export async function searchPosts(request: SearchPostsRequest): Promise<GetPostsResponse> {
	try {
		const response = await publicApi.post('v0/posts/search', { json: request });
		return response.json<GetPostsResponse>();
	} catch (error) {
		console.error('Failed to search posts:', error);
		throw error;
	}
}

export async function uploadThumbnail(request: ThumbnailUploadRequest): Promise<void> {
	try {
		const formData = new FormData();
		formData.append('post_id', request.post_id);
		formData.append('file', request.file);

		await privateApi.post('v0/post/thumbnail', {
			body: formData,
			headers: {
				'Content-Type': undefined
			}
		});
	} catch (error) {
		console.error('Failed to upload thumbnail:', error);
		throw error;
	}
}

export async function incrementPostView(request: GetPostByUuidRequest): Promise<void> {
	try {
		await publicApi.post('v0/post/view', { json: request });
	} catch (error) {
		console.error('Failed to increment post view:', error);
		throw error;
	}
}

export async function getUserPosts(request: GetUserPostsRequest): Promise<UserPostsResponse> {
	try {
		const response = await publicApi.post('v0/posts/user', { json: request });
		return response.json<UserPostsResponse>();
	} catch (error) {
		console.error('Failed to get user posts:', error);
		throw error;
	}
}

export async function updatePost(request: UpdatePostRequest): Promise<void> {
	try {
		await privateApi.put('v0/post', { json: request });
	} catch (error) {
		console.error('Failed to update post:', error);
		throw error;
	}
}

export async function deletePost(request: DeletePostRequest): Promise<void> {
	try {
		await privateApi.delete('v0/post', { json: request });
	} catch (error) {
		console.error('Failed to delete post:', error);
		throw error;
	}
}

export async function getPostForEdit(request: GetPostForEditRequest): Promise<PostEditInfoResponse> {
	try {
		const response = await privateApi.post('v0/post/edit', { json: request });
		return response.json<PostEditInfoResponse>();
	} catch (error) {
		console.error('Failed to get post for edit:', error);
		throw error;
	}
}

export async function uploadImage(request: ImageUploadRequest): Promise<PostImageUploadResponse> {
	try {
		const formData = new FormData();
		formData.append('file', request.file);

		const response = await privateApi.post('v0/post/image', {
			body: formData,
			headers: {
				'Content-Type': undefined
			}
		});

		return response.json<PostImageUploadResponse>();
	} catch (error) {
		console.error('Failed to upload image:', error);
		throw error;
	}
}
