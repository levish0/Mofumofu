import { getComments, getReplies } from '$lib/api/comment/commentApi';
import type { CommentInfo, GetCommentsRequest, GetRepliesRequest } from '$lib/api/comment/types';

interface UseCommentsState {
	comments: CommentInfo[];
	currentPage: number;
	loading: boolean;
	hasMore: boolean;
	totalCount: number;
}

export function useComments(postId: string, pageSize: number = 5) {
	let state = $state<UseCommentsState>({
		comments: [],
		currentPage: 1,
		loading: false,
		hasMore: false,
		totalCount: 0
	});

	const loadComments = async (page: number = 1, reset: boolean = false) => {
		if (state.loading) return;

		try {
			state.loading = true;

			const request: GetCommentsRequest = {
				post_id: postId,
				page,
				per_page: pageSize
			};

			const response = await getComments(request);

			if (reset) {
				state.comments = response.comments;
			} else {
				state.comments.push(...response.comments);
			}

			state.currentPage = response.page;
			state.hasMore = response.has_next;
			state.totalCount = response.total_count;
		} catch (error) {
			console.error('Failed to load comments:', error);
		} finally {
			state.loading = false;
		}
	};

	const loadMoreComments = async () => {
		if (state.hasMore && !state.loading) {
			await loadComments(state.currentPage + 1, false);
		}
	};

	const refreshComments = async () => {
		await loadComments(1, true);
	};

	const addComment = (newComment: CommentInfo) => {
		state.comments.unshift(newComment);
		state.totalCount++;
	};

	return {
		get comments() {
			return state.comments;
		},
		get loading() {
			return state.loading;
		},
		get hasMore() {
			return state.hasMore;
		},
		get totalCount() {
			return state.totalCount;
		},
		loadComments,
		loadMoreComments,
		refreshComments,
		addComment
	};
}
