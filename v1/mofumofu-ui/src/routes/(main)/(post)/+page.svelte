<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { usePostsData } from '$lib/hooks/posts/usePostsData.svelte';
	import { postsStore } from '$lib/stores/posts.svelte';

	const PAGE_SIZE = 15;
	const skeletonCount = 5;

	// 커스텀 훅 사용
	const { loadMorePosts } = usePostsData({
		pageSize: PAGE_SIZE,
		sortOrder: 'popular'
	});

	// Store에서 직접 reactive 값 가져오기
	const posts = $derived(postsStore.posts);
	const loading = $derived(postsStore.loading);
	const hasMore = $derived(postsStore.hasMore);
</script>

<svelte:head>
	<title>Mofumofu - A soft and simple blogging platform for everyone.</title>
	<meta
		name="description"
		content="Mofumofu is an open-source, minimalist blogging platform. Write essays, tutorials, or daily thoughts and focus on what matters most."
	/>

	<!-- Open Graph -->
	<meta property="og:title" content="Mofumofu - A soft and simple blogging platform for everyone" />
	<meta
		property="og:description"
		content="Mofumofu is an open-source, minimalist blogging platform. Write essays, tutorials, or daily thoughts and focus on what matters most."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://mofumofu.ink" />
	<meta property="og:image" content="https://mofumofu.ink/og-default.png" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Mofumofu - A soft and simple blogging platform for everyone" />
	<meta
		name="twitter:description"
		content="Mofumofu is an open-source, minimalist blogging platform. Write essays, tutorials, or daily thoughts and focus on what matters most."
	/>
	<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />

	<link rel="canonical" href="https://mofumofu.ink" />
</svelte:head>

<PostList {posts} {loading} onLoadMore={loadMorePosts} {hasMore} {skeletonCount} />
