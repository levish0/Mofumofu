<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { Input } from '$lib/components/ui/input';
	import { useSearchData } from '$lib/hooks/search/useSearchData.svelte';
	import { searchStore } from '$lib/stores/search.svelte';

	const skeletonCount = 5;

	// 검색 데이터 훅 사용 (액션만)
	const { startSearch, loadMorePosts } = useSearchData({ pageSize: 15 });

	// Store에서 직접 reactive 값 가져오기
	const searchQuery = $derived(searchStore.searchQuery);
	const posts = $derived(searchStore.posts);
	const loading = $derived(searchStore.loading);
	const hasMore = $derived(searchStore.hasMore);
	const totalResults = $derived(searchStore.totalResults);
	const hasSearched = $derived(searchStore.hasSearched);

	// 로컬 검색 쿼리 상태 (입력 필드용)
	let inputQuery = $state('');
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	// 검색 쿼리가 변경되면 입력 필드도 업데이트
	$effect(() => {
		inputQuery = searchQuery;
	});

	// Store 상태 디버깅
	$effect(() => {
		console.log('Search page state:', {
			searchQuery,
			posts: posts.length,
			loading,
			hasMore,
			totalResults,
			hasSearched
		});
	});

	// 디바운싱된 검색
	$effect(() => {
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		if (inputQuery.trim()) {
			debounceTimer = setTimeout(() => {
				startSearch(inputQuery);
			}, 250);
		}

		return () => {
			if (debounceTimer) {
				clearTimeout(debounceTimer);
			}
		};
	});

	function handleInputKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (debounceTimer) {
				clearTimeout(debounceTimer);
			}
			if (inputQuery.trim()) {
				startSearch(inputQuery);
			}
		}
	}
</script>

<svelte:head>
	<title>Search - Mofu</title>
</svelte:head>

<div class="min-h-screen">
	<div class="w-full px-4 pt-8">
		<!-- Search Input Section -->
		<div class="mb-8">
			<div class="relative mx-auto max-w-2xl">
				<div class="relative">
					<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<svg class="text-mofu-dark-300 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
							></path>
						</svg>
					</div>
					<Input
						type="text"
						bind:value={inputQuery}
						onkeydown={handleInputKeydown}
						placeholder="Search posts..."
						class="dark:bg-mofu-dark-800 dark:text-mofu-dark-300 dark:placeholder-mofu-dark-400  w-full rounded-xl bg-white py-3 pr-4 pl-10  text-lg text-gray-900 placeholder-gray-500"
					/>
				</div>
			</div>
		</div>

		<!-- Results Count -->
		{#if hasSearched && !loading && posts.length > 0}
			<div class="dark:text-mofu-dark-400 text-mofu-light-700 mb-6 text-center">
				총 {totalResults}개의 포스트를 찾았습니다.
			</div>
		{/if}

		<!-- No Results -->
		{#if hasSearched && !loading && posts.length === 0}
			<div class=" text-center">
				<div class="dark:text-mofu-dark-400 text-lg text-gray-500">
					"{searchQuery}"에 대한 검색 결과가 없습니다.
				</div>
			</div>
		{/if}

		<!-- Search Results -->
		{#if posts.length > 0 || (loading && hasSearched)}
			<PostList {posts} {loading} onLoadMore={loadMorePosts} {hasMore} {skeletonCount} />
		{/if}

		<!-- Welcome Message -->
		{#if !hasSearched && !loading}
			<div class="py-12 text-center">
				<div class="mb-4 text-lg text-gray-500 dark:text-gray-400">포스트를 검색해보세요</div>
				<div class="text-gray-400 dark:text-gray-500">제목, 내용, 해시태그, 작성자로 검색할 수 있습니다</div>
			</div>
		{/if}
	</div>
</div>
