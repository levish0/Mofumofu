<script lang="ts">
	import type { FollowUserItem } from '$lib/api/types';
	import FollowUserItemComponent from './follow-user-item.svelte';

	let {
		users = [],
		loading = false,
		hasMore = false,
		onLoadMore
	}: {
		users?: FollowUserItem[];
		loading?: boolean;
		hasMore?: boolean;
		onLoadMore?: () => void;
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

<div class="space-y-1">
	{#each users as user (user.id)}
		<FollowUserItemComponent {user} />
	{/each}

	{#if loading}
		{#each Array(3) as _}
			<div class="flex animate-pulse items-center gap-3 px-3 py-3">
				<div class="h-11 w-11 rounded-full bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
				<div class="flex-1 space-y-2">
					<div class="h-4 w-32 rounded bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
					<div class="h-3 w-24 rounded bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
				</div>
			</div>
		{/each}
	{/if}
</div>
