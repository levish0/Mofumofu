import { createLike, deleteLike, checkLikeStatus } from '$lib/api/like/likeApi';
import { createCommentLike, deleteCommentLike, checkCommentLikeStatus } from '$lib/api/comment/commentApi';
import { authStore } from '$lib/stores/auth.svelte';
import { toast } from 'svelte-sonner';
import { onMount } from 'svelte';
import * as m from '../../paraglide/messages';

type LikeType = 'post' | 'comment';

interface UseLikeOptions {
	id: string;
	type: LikeType;
	initialCount: number;
	onUpdate?: (id: string, isLiked: boolean) => void;
}

export function useLike(options: UseLikeOptions) {
	const { id, type, initialCount, onUpdate } = options;

	let isLiked = $state(false);
	let likeCount = $state(initialCount || 0);
	let isLoading = $state(true);
	let isSubmitting = $state(false);

	// API 함수들 선택
	const apiMethods = {
		post: {
			create: (id: string) => createLike({ post_id: id }),
			delete: (id: string) => deleteLike({ post_id: id }),
			checkStatus: (id: string) => checkLikeStatus({ post_id: id })
		},
		comment: {
			create: (id: string) => createCommentLike({ comment_id: id }),
			delete: (id: string) => deleteCommentLike({ comment_id: id }),
			checkStatus: (id: string) => checkCommentLikeStatus({ comment_id: id })
		}
	};

	const api = apiMethods[type];

	// 좋아요 상태 로드
	async function loadLikeStatus() {
		if (!authStore.isAuthenticated) {
			isLoading = false;
			return;
		}

		try {
			const response = await api.checkStatus(id);
			isLiked = response.is_liked;
		} catch (error) {
			console.error('Failed to check like status:', error);
		} finally {
			isLoading = false;
		}
	}

	// 좋아요 토글
	async function toggleLike() {
		if (!authStore.isAuthenticated) {
			return;
		}

		if (isSubmitting) return;

		isSubmitting = true;
		try {
			if (isLiked) {
				await api.delete(id);
				isLiked = false;
				likeCount = Math.max(0, likeCount - 1);
			} else {
				await api.create(id);
				isLiked = true;
				likeCount += 1;
			}

			// 콜백 호출 (댓글용)
			onUpdate?.(id, isLiked);
		} catch (error) {
			console.error(`Failed to toggle ${type} like:`, error);
			toast.error(m.like_processing_error());
		} finally {
			isSubmitting = false;
		}
	}

	// 초기화
	onMount(() => {
		loadLikeStatus();
	});

	// 외부에서 상태 업데이트 (댓글용)
	function updateLikeState(liked: boolean) {
		isLiked = liked;
	}

	// 카운트 업데이트
	function updateLikeCount(count: number) {
		likeCount = count;
	}

	return {
		isLiked: () => isLiked,
		likeCount: () => likeCount,
		isLoading: () => isLoading,
		isSubmitting: () => isSubmitting,
		toggleLike,
		updateLikeState,
		updateLikeCount
	};
}
