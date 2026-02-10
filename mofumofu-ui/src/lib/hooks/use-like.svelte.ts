import { createLike, deleteLike, checkLikeStatus } from '$lib/api/likes/likesApi';
import type { LikeTargetType } from '$lib/api/types';
import { toast } from 'svelte-sonner';
import { onMount } from 'svelte';

interface UseLikeOptions {
	id: string;
	type: LikeTargetType;
	initialCount: number;
	isAuthenticated: () => boolean;
}

export function useLike(options: UseLikeOptions) {
	const { id, type, initialCount, isAuthenticated } = options;

	let isLiked = $state(false);
	let likeCount = $state(initialCount);
	let isLoading = $state(true);
	let isSubmitting = $state(false);

	async function loadLikeStatus() {
		if (!isAuthenticated()) {
			isLoading = false;
			return;
		}

		try {
			const response = await checkLikeStatus(id, type);
			isLiked = response.liked;
		} catch (error) {
			console.error('Failed to check like status:', error);
		} finally {
			isLoading = false;
		}
	}

	async function toggleLike() {
		if (!isAuthenticated() || isSubmitting) return;

		isSubmitting = true;
		try {
			if (isLiked) {
				await deleteLike(id, type);
				isLiked = false;
				likeCount = Math.max(0, likeCount - 1);
			} else {
				await createLike(id, type);
				isLiked = true;
				likeCount += 1;
			}
		} catch (error) {
			console.error(`Failed to toggle ${type} like:`, error);
			toast.error('Failed to process like.');
		} finally {
			isSubmitting = false;
		}
	}

	onMount(() => {
		loadLikeStatus();
	});

	return {
		isLiked: () => isLiked,
		likeCount: () => likeCount,
		isLoading: () => isLoading,
		isSubmitting: () => isSubmitting,
		toggleLike
	};
}
