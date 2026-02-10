<script lang="ts">
	import type { PostSearchItem, PostFeedItem } from '$lib/api/types';
	import PostCard from './post-card.svelte';

	let {
		posts = [],
		loading = false,
		hasMore = false,
		onLoadMore,
		skeletonCount = 8
	}: {
		posts?: (PostSearchItem | PostFeedItem)[];
		loading?: boolean;
		hasMore?: boolean;
		onLoadMore?: () => void;
		skeletonCount?: number;
	} = $props();

	let rafId: number | null = null;

	function handleScroll() {
		if (rafId) return;

		rafId = requestAnimationFrame(() => {
			rafId = null;

			if (loading || !hasMore || !onLoadMore) return;

			const { scrollTop, clientHeight, scrollHeight } = document.documentElement;
			if (scrollTop + clientHeight >= scrollHeight - 200) {
				onLoadMore();
			}
		});
	}

	$effect(() => {
		window.addEventListener('scroll', handleScroll, { passive: true });
		return () => {
			window.removeEventListener('scroll', handleScroll);
			if (rafId) cancelAnimationFrame(rafId);
		};
	});
</script>

<div class="grid grid-cols-1 gap-x-5 gap-y-4 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-5">
	{#each posts as post (post.id)}
		<PostCard {post} />
	{/each}

	{#if loading}
		{#each Array(skeletonCount) as _}
			<PostCard />
		{/each}
	{/if}
</div>
