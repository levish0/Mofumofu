<script lang="ts">
	import { Heart, Icon } from 'svelte-hero-icons';
	import { useLike } from '$lib/hooks/useLike.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';

	interface Props {
		commentId: string;
		initialLikeCount: number;
		isDeleted: boolean;
		onLikeUpdate?: (commentId: string, isLiked: boolean) => void;
	}

	let { commentId, initialLikeCount, isDeleted, onLikeUpdate }: Props = $props();

	const like = useLike({
		id: commentId,
		type: 'comment',
		initialCount: initialLikeCount,
		onUpdate: onLikeUpdate
	});

	const isAuthenticated = $derived(authStore.isAuthenticated);
</script>

{#if like.isLoading()}
	<div class="shimmer h-8 w-16 rounded-full"></div>
{:else}
	<button
		onclick={(e) => {
			e.stopPropagation();
			like.toggleLike();
		}}
		disabled={like.isSubmitting() || isDeleted || !isAuthenticated}
		class="flex items-center gap-2 rounded-full px-4 py-2 transition-colors {like.isLiked()
			? 'text-rose-600 dark:text-rose-500'
			: 'dark:text-mofu-dark-400 text-mofu-light-400'} 
		{like.isSubmitting() || !isAuthenticated
			? 'cursor-not-allowed opacity-50'
			: 'hover:text-rose-600 dark:hover:text-rose-500'}"
	>
		<Icon src={Heart} class="h-5 w-5" solid />
		<span class="text-sm">{like.likeCount()}</span>
		{#if like.isSubmitting()}
			<div
				class="border-mofu-light-400 dark:border-mofu-dark-400 h-3 w-3 animate-spin rounded-full border-2 border-t-transparent"
			></div>
		{/if}
	</button>
{/if}
