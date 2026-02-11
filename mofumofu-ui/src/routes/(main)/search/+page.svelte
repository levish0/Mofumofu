<script lang="ts">
	import { searchPosts } from '$lib/api/posts';
	import type { PostSearchItem } from '$lib/api/types';
	import { PostList } from '$lib/components/post';
	import FlatInput from '$lib/components/flat-input/flat-input.svelte';
	import { Icon, MagnifyingGlass } from 'svelte-hero-icons';

	const PAGE_SIZE = 15;

	let query = $state('');
	let posts = $state<PostSearchItem[]>([]);
	let pageNum = $state(1);
	let totalPages = $state(1);
	let totalHits = $state(0);
	let loading = $state(false);
	let hasSearched = $state(false);
	let hasMore = $derived(pageNum < totalPages);

	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	async function search(q: string, page: number) {
		loading = true;
		try {
			const res = await searchPosts({
				query: q,
				page,
				page_size: PAGE_SIZE,
				sort_by: 'CreatedAt',
				sort_order: 'Desc'
			});
			posts = page === 1 ? res.posts : [...posts, ...res.posts];
			totalPages = res.total_pages;
			totalHits = res.total_hits;
			pageNum = page;
			hasSearched = true;
		} finally {
			loading = false;
		}
	}

	function startSearch(q: string) {
		if (!q.trim()) return;
		posts = [];
		pageNum = 1;
		search(q.trim(), 1);
	}

	function loadMore() {
		search(query.trim(), pageNum + 1);
	}

	$effect(() => {
		if (debounceTimer) clearTimeout(debounceTimer);

		if (query.trim()) {
			debounceTimer = setTimeout(() => startSearch(query), 300);
		}

		return () => {
			if (debounceTimer) clearTimeout(debounceTimer);
		};
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			if (debounceTimer) clearTimeout(debounceTimer);
			startSearch(query);
		}
	}
</script>

<svelte:head>
	<title>Search - Mofumofu</title>
</svelte:head>

<div class="mx-auto min-h-screen max-w-8xl px-4 py-6">
	<!-- Search input -->
	<div class="mx-auto mb-8 max-w-2xl">
		<div class="relative">
			<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
				<Icon src={MagnifyingGlass} class="h-5 w-5 text-mofu-light-300 dark:text-mofu-dark-300" />
			</div>
			<FlatInput
				type="text"
				bind:value={query}
				onkeydown={handleKeydown}
				placeholder="Search posts..."
				class="py-3 pl-10 text-lg"
			/>
		</div>
	</div>

	<!-- Results info -->
	{#if hasSearched && !loading && posts.length > 0}
		<p class="mb-4 text-center text-sm text-mofu-light-300 dark:text-mofu-dark-300">
			{totalHits.toLocaleString()} result{totalHits !== 1 ? 's' : ''} found
		</p>
	{/if}

	<!-- No results -->
	{#if hasSearched && !loading && posts.length === 0}
		<div class="py-16 text-center">
			<p class="text-lg text-mofu-light-300 dark:text-mofu-dark-300">
				No results for "{query}"
			</p>
		</div>
	{/if}

	<!-- Results -->
	{#if posts.length > 0 || (loading && hasSearched)}
		<PostList {posts} {loading} {hasMore} onLoadMore={loadMore} />
	{/if}

	<!-- Welcome -->
	{#if !hasSearched && !loading}
		<div class="py-20 text-center">
			<Icon
				src={MagnifyingGlass}
				class="mx-auto mb-4 h-12 w-12 text-mofu-light-400 dark:text-mofu-dark-400"
			/>
			<p class="text-lg text-mofu-light-300 dark:text-mofu-dark-300">
				Search by title, content, hashtag, or author
			</p>
		</div>
	{/if}
</div>
