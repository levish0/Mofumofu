import { searchPosts } from '$lib/api';
import type { SearchPostsRequest, PostListItem } from '$lib/api/post/types';
import { searchStore } from '$lib/stores/search.svelte';

interface UseSearchDataConfig {
	pageSize?: number;
}

export function useSearchData(config: UseSearchDataConfig = {}) {
	const PAGE_SIZE = config.pageSize ?? 15;

	// API 호출 파라미터 빌드
	const buildApiParams = (query: string, page: number): SearchPostsRequest => {
		return {
			query: query.trim(),
			page,
			page_size: PAGE_SIZE
		};
	};

	// 검색 실행
	const performSearch = async (query: string, page = 1, resetResults = true) => {
		if (!query.trim()) return;

		console.log('performSearch called with:', { query, page, resetResults });
		searchStore.setLoading(true);

		try {
			const params = buildApiParams(query, page);
			console.log('API params:', params);
			const response = await searchPosts(params);
			console.log('API response:', response);

			if (resetResults) {
				console.log('Setting posts:', response.posts);
				searchStore.setPosts(response.posts);
				searchStore.setCurrentPage(1);
				searchStore.setHasSearched(true);
			} else {
				// 중복 제거 후 추가
				const existingKeys = new Set(searchStore.posts.map((post) => `${post.user_handle}-${post.slug}`));
				const newPosts = response.posts.filter((post) => !existingKeys.has(`${post.user_handle}-${post.slug}`));
				console.log('Adding posts:', newPosts);
				searchStore.addPosts(newPosts);
			}

			searchStore.setHasMore(response.has_more);
			searchStore.setCurrentPage(response.current_page);
			searchStore.setTotalResults(response.total_count || 0);
			console.log('Updated store state:', {
				posts: searchStore.posts.length,
				hasMore: response.has_more,
				currentPage: response.current_page,
				totalResults: response.total_count
			});
		} catch (error) {
			console.error('Search failed:', error);
			if (resetResults) {
				searchStore.setPosts([]);
				searchStore.setHasMore(false);
				searchStore.setTotalResults(0);
			}
		} finally {
			searchStore.setLoading(false);
		}
	};

	// 다음 페이지 로드 (무한스크롤)
	const loadMorePosts = async () => {
		if (searchStore.loading || !searchStore.hasMore || !searchStore.searchQuery.trim()) return;

		const nextPage = searchStore.currentPage + 1;
		await performSearch(searchStore.searchQuery, nextPage, false);
	};

	// 새로운 검색 시작
	const startSearch = async (query: string) => {
		searchStore.setSearchQuery(query);
		await performSearch(query);
	};

	// 검색 상태 리셋
	const resetSearch = () => {
		searchStore.reset();
	};

	// 모든 상태 클리어
	const clearAll = () => {
		searchStore.clear();
	};

	return {
		// Actions only
		startSearch,
		loadMorePosts,
		resetSearch,
		clearAll,
		setSearchQuery: searchStore.setSearchQuery.bind(searchStore)
	};
}
