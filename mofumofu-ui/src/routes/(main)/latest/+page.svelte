<script lang="ts">
	import { PostList } from '$lib/components/post';
	import { getPostFeed } from '$lib/api/posts';
	import type { PostFeedItem } from '$lib/api/types';

	const PAGE_SIZE = 15;

	let posts = $state<PostFeedItem[]>([]);
	let page = $state(1);
	let hasMore = $state(false);
	let loading = $state(true);

	async function loadPosts(pageNum: number) {
		loading = true;
		try {
			const res = await getPostFeed({
				sort: 'Latest',
				page: pageNum,
				page_size: PAGE_SIZE
			});
			posts = pageNum === 1 ? res.data : [...posts, ...res.data];
			hasMore = res.has_more;
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
