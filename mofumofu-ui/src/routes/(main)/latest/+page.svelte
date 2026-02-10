<script lang="ts">
	import { PostList } from '$lib/components/post';
	import { searchPosts } from '$lib/api/posts';
	import type { PostSearchItem } from '$lib/api/types';

	const PAGE_SIZE = 15;

	let posts = $state<PostSearchItem[]>([]);
	let page = $state(1);
	let totalPages = $state(1);
	let loading = $state(true);
	let hasMore = $derived(page < totalPages);

	async function loadPosts(pageNum: number) {
		loading = true;
		try {
			const res = await searchPosts({
				page: pageNum,
				page_size: PAGE_SIZE,
				sort_by: 'CreatedAt',
				sort_order: 'Desc'
			});
			posts = pageNum === 1 ? res.posts : [...posts, ...res.posts];
			totalPages = res.total_pages;
		} finally {
			loading = false;
		}
	}

	function loadMore() {
		page += 1;
		loadPosts(page);
	}

	$effect(() => {
		loadPosts(1);
	});
</script>

<div class="mx-auto max-w-8xl px-4 py-6">
	<PostList {posts} {loading} {hasMore} onLoadMore={loadMore} />
</div>
