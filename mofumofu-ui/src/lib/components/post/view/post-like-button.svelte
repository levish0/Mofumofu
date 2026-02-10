<script lang="ts">
	import { Icon, Heart } from 'svelte-hero-icons';
	import { useLike } from '$lib/hooks/use-like.svelte';

	interface Props {
		postId: string;
		initialLikeCount: number;
		isAuthenticated: boolean;
	}

	const { postId, initialLikeCount, isAuthenticated }: Props = $props();

	const like = useLike({
		id: postId,
		type: 'Post',
		initialCount: initialLikeCount,
		isAuthenticated: () => isAuthenticated
	});
</script>

{#if like.isLoading()}
	<div class="shimmer h-6 w-10 rounded"></div>
{:else}
	<button
		onclick={like.toggleLike}
		disabled={like.isSubmitting() || !isAuthenticated}
		class="flex items-center gap-2 rounded-full px-4 py-2 transition-colors {like.isLiked()
			? 'text-rose-600 dark:text-rose-500'
			: 'text-mofu-light-400 dark:text-mofu-dark-400'}
		{like.isSubmitting() || !isAuthenticated
			? 'cursor-not-allowed opacity-50'
			: 'hover:text-rose-600 dark:hover:text-rose-500'}"
	>
		<Icon src={Heart} class="h-5 w-5" solid />
		<span class="text-sm">{like.likeCount()}</span>
	</button>
{/if}
