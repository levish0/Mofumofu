<script lang="ts">
	import type { PostSearchItem } from '$lib/api/types';
	import PostCard from './post-card.svelte';

	let {
		posts = [],
		loading = false,
		hasMore = false,
		onLoadMore,
		skeletonCount = 8
	}: {
		posts?: PostSearchItem[];
		loading?: boolean;
		hasMore?: boolean;
		onLoadMore?: () => void;
		skeletonCount?: number;
	} = $props();

	let sentinel = $state<HTMLDivElement | null>(null);

	$effect(() => {
		if (!sentinel) return;

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting && hasMore && !loading && onLoadMore) {
					onLoadMore();
				}
			},
			{ rootMargin: '200px' }
		);

		observer.observe(sentinel);

		return () => observer.disconnect();
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

{#if hasMore && !loading}
	<div bind:this={sentinel} class="h-1"></div>
{/if}
