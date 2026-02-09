<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { usePostsData } from '$lib/hooks/posts/usePostsData.svelte';
	import { postsStore } from '$lib/stores/posts.svelte';

	const PAGE_SIZE = 15;
	const skeletonCount = 5;

	// 커스텀 훅 사용
	const { loadMorePosts } = usePostsData({
		pageSize: PAGE_SIZE,
		sortOrder: 'latest'
	});

	// Store에서 직접 reactive 값 가져오기
	const posts = $derived(postsStore.posts);
	const loading = $derived(postsStore.loading);
	const hasMore = $derived(postsStore.hasMore);
</script>

<svelte:head>
	<title>최신 포스트 - Mofumofu</title>
	<meta name="description" content="Mofumofu에서 최신으로 작성된 포스트들을 확인해보세요." />

	<!-- Open Graph -->
	<meta property="og:title" content="최신 포스트 - Mofumofu" />
	<meta property="og:description" content="Mofumofu에서 최신으로 작성된 포스트들을 확인해보세요." />
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://mofumofu.ink/latest" />
	<meta property="og:image" content="https://mofumofu.ink/og-default.png" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="최신 포스트 - Mofumofu" />
	<meta name="twitter:description" content="Mofumofu에서 최신으로 작성된 포스트들을 확인해보세요." />
	<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />

	<link rel="canonical" href="https://mofumofu.ink/latest" />
</svelte:head>

<PostList {posts} {loading} onLoadMore={loadMorePosts} {hasMore} {skeletonCount} />
