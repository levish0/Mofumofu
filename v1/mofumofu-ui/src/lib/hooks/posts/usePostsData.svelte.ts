import { getPosts } from '$lib/api/post/postApi';
import type { PostSortOrder, GetPostsRequest, PostListItem } from '$lib/api/post/types';
import { postsStore } from '$lib/stores/posts.svelte';
import { onMount } from 'svelte';

interface UsePostsDataConfig {
	pageSize?: number;
	sortOrder?: PostSortOrder;
}

export function usePostsData(config: UsePostsDataConfig = {}) {
	const PAGE_SIZE = config.pageSize ?? 8;
	const SORT_ORDER = config.sortOrder ?? 'latest';

	// API 호출 파라미터 빌드
	const buildApiParams = (page: number) => {
		return {
			page: page,
			page_size: PAGE_SIZE,
			sort: SORT_ORDER
		};
	};

	// API 호출 함수 - 모든 post 요청에 getPosts 사용
	const callApi = async (page: number) => {
		const params = buildApiParams(page);
		return await getPosts(params as GetPostsRequest);
	};

	// 초기 데이터 로드 - Store에 저장된 포스트가 있으면 복원, 없으면 targetPage부터 로드
	const loadInitialPosts = async () => {
		const targetPage = postsStore.targetPage;
		const existingPosts = postsStore.posts;

		// 이미 저장된 포스트가 있으면 복원
		if (existingPosts.length > 0) {
			postsStore.setInitialLoading(false);
			return;
		}

		// 저장된 포스트가 없으면 targetPage부터 누적 로드
		try {
			postsStore.setInitialLoading(true);

			const allPosts: PostListItem[] = [];
			let hasMore = true;
			let currentPage = 1;

			for (let page = 1; page <= targetPage; page++) {
				const response = await callApi(page);
				allPosts.push(...response.posts);
				currentPage = response.current_page;
				hasMore = response.has_more;
			}

			postsStore.setPosts(allPosts);
			postsStore.setHasMore(hasMore);
			postsStore.setCurrentPage(currentPage);
		} catch (error) {
			console.error('Failed to load initial posts:', error);
			postsStore.setPosts([]);
			postsStore.setHasMore(false);
		} finally {
			postsStore.setInitialLoading(false);
		}
	};

	// 다음 페이지 로드 (무한스크롤)
	const loadMorePosts = async () => {
		if (postsStore.loading || !postsStore.hasMore) return;

		postsStore.setLoading(true);

		try {
			const nextPage = postsStore.currentPage + 1;
			const response = await callApi(nextPage);

			// 중복 제거 후 추가 (user_handle + slug 조합으로 고유성 판단)
			const existingKeys = new Set(postsStore.posts.map((post) => `${post.user_handle}-${post.slug}`));
			const newPosts = response.posts.filter((post) => !existingKeys.has(`${post.user_handle}-${post.slug}`));

			postsStore.addPosts(newPosts);
			postsStore.setCurrentPage(response.current_page);
			postsStore.setTargetPage(response.current_page);
			postsStore.setHasMore(response.has_more);
		} catch (error) {
			console.error('Failed to load more posts:', error);
		} finally {
			postsStore.setLoading(false);
		}
	};

	// 데이터 리로드
	const reloadData = () => {
		postsStore.reset();
		loadInitialPosts();
	};

	// 컴포넌트 마운트 시 초기 로드
	onMount(() => {
		loadInitialPosts();
	});

	return {
		// Store getters
		get posts() {
			return postsStore.posts;
		},
		get loading() {
			return postsStore.loading;
		},
		get initialLoading() {
			return postsStore.initialLoading;
		},
		get hasMore() {
			return postsStore.hasMore;
		},
		get currentPage() {
			return postsStore.currentPage;
		},
		// Actions
		loadInitialPosts,
		loadMorePosts,
		reloadData
	};
}
