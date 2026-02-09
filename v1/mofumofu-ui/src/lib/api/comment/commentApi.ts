import { privateApi } from '../private';
import type {
	CreateCommentRequest,
	CreateCommentResponse,
	UpdateCommentRequest,
	DeleteCommentRequest,
	GetCommentByIdRequest,
	GetCommentsRequest,
	GetRepliesRequest,
	CommentInfo,
	GetCommentsResponse,
	GetRepliesResponse,
	CreateCommentLikeRequest,
	DeleteCommentLikeRequest,
	CheckCommentLikeStatusRequest
} from './types';
import type { LikeStatusResponse } from '../like/types';
import { publicApi } from '../public';

export async function createComment(request: CreateCommentRequest): Promise<CreateCommentResponse> {
	try {
		const response = await privateApi.post('v0/comment', { json: request });
		return response.json<CreateCommentResponse>();
	} catch (error) {
		console.error('Failed to create comment:', error);
		throw error;
	}
}

export async function updateComment(request: UpdateCommentRequest): Promise<void> {
	try {
		await privateApi.put('v0/comment', { json: request });
	} catch (error) {
		console.error('Failed to update comment:', error);
		throw error;
	}
}

export async function deleteComment(request: DeleteCommentRequest): Promise<void> {
	try {
		await privateApi.delete('v0/comment', { json: request });
	} catch (error) {
		console.error('Failed to delete comment:', error);
		throw error;
	}
}

export async function getCommentById(request: GetCommentByIdRequest): Promise<CommentInfo> {
	try {
		const response = await publicApi.post('v0/comment/get', { json: request });
		return response.json<CommentInfo>();
	} catch (error) {
		console.error('Failed to get comment:', error);
		throw error;
	}
}

export async function getComments(request: GetCommentsRequest): Promise<GetCommentsResponse> {
	try {
		const response = await publicApi.post('v0/comment/list', { json: request });
		return response.json<GetCommentsResponse>();
	} catch (error) {
		console.error('Failed to get comments:', error);
		throw error;
	}
}

export async function getReplies(request: GetRepliesRequest): Promise<GetRepliesResponse> {
	try {
		const response = await publicApi.post('v0/comment/replies', { json: request });
		return response.json<GetRepliesResponse>();
	} catch (error) {
		console.error('Failed to get replies:', error);
		throw error;
	}
}

export async function createCommentLike(request: CreateCommentLikeRequest): Promise<void> {
	try {
		await privateApi.post('v0/comment/like', { json: request });
	} catch (error) {
		console.error('Failed to like comment:', error);
		throw error;
	}
}

export async function deleteCommentLike(request: DeleteCommentLikeRequest): Promise<void> {
	try {
		await privateApi.delete('v0/comment/like', { json: request });
	} catch (error) {
		console.error('Failed to unlike comment:', error);
		throw error;
	}
}

export async function checkCommentLikeStatus(request: CheckCommentLikeStatusRequest): Promise<LikeStatusResponse> {
	try {
		const response = await privateApi.post('v0/comment/like/status', { json: request });
		return response.json<LikeStatusResponse>();
	} catch (error) {
		console.error('Failed to check comment like status:', error);
		throw error;
	}
}
