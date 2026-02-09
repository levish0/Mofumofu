import { searchPosts } from '$lib/api/post/postApi';
import type { PostSortOrder, PostListItem } from '$lib/api/post/types';
import { profilePostsStore } from '$lib/stores/profilePosts.svelte';

interface UseProfilePostsDataConfig {
	pageSize?: number;
}

export function useProfilePostsData(config: UseProfilePostsDataConfig = {}) {
	const PAGE_SIZE = config.pageSize ?? 12;

	// 초기 데이터 로드 - Store에 저장된 포스트가 있으면 복원, 없으면 targetPage부터 로드
	const loadInitialPosts = async (userHandle: string, sortOrder: PostSortOrder = 'latest') => {
		if (!userHandle) return;

		const targetPage = profilePostsStore.targetPage;
		const existingPosts = profilePostsStore.posts;
		const currentUserHandle = profilePostsStore.userHandle;
		const currentSortOrder = profilePostsStore.sortOrder;

		console.log(
			'Profile loadInitialPosts called, userHandle:',
			userHandle,
			'targetPage:',
			targetPage,
			'existing posts:',
			existingPosts.length
		);

		// 같은 사용자, 같은 정렬이고 이미 저장된 포스트가 있으면 그대로 사용 (페이지 복원)
		if (existingPosts.length > 0 && currentUserHandle === userHandle && currentSortOrder === sortOrder) {
			console.log('Using existing profile posts from store');
			profilePostsStore.setInitialLoading(false);
			return;
		}

		// 저장된 포스트가 없거나 다른 사용자/정렬이면 새로 로드
		try {
			profilePostsStore.setInitialLoading(true);
			profilePostsStore.setUserHandle(userHandle);
			profilePostsStore.setSortOrder(sortOrder);

			// targetPage까지 순차적으로 로드하여 누적
			const allPosts: PostListItem[] = [];
			let hasMore = true;
			let currentPage = 1;

			for (let page = 1; page <= targetPage; page++) {
				const response = await searchPosts({
					user_handle: userHandle,
					sort: sortOrder,
					page_size: PAGE_SIZE,
					page: page
				});
				allPosts.push(...response.posts);
				currentPage = response.current_page;
				hasMore = response.has_more;
			}

			profilePostsStore.setPosts(allPosts);
			profilePostsStore.setHasMore(hasMore);
			profilePostsStore.setCurrentPage(currentPage);
			profilePostsStore.setInitialized(true);

			console.log('Profile loadInitialPosts completed: loaded', allPosts.length, 'posts up to page', targetPage);
		} catch (error) {
			console.error('Failed to load profile posts:', error);
			profilePostsStore.setPosts([]);
			profilePostsStore.setHasMore(false);
		} finally {
			profilePostsStore.setInitialLoading(false);
		}
	};

	// 다음 페이지 로드 (아래쪽 무한스크롤)
	const loadMorePosts = async () => {
		if (profilePostsStore.loading || !profilePostsStore.hasMore || !profilePostsStore.userHandle) {
			return;
		}

		console.log('Profile loadMorePosts called');
		profilePostsStore.setLoading(true);

		try {
			const nextPage = profilePostsStore.currentPage + 1;
			const response = await searchPosts({
				user_handle: profilePostsStore.userHandle,
				sort: profilePostsStore.sortOrder,
				page_size: PAGE_SIZE,
				page: nextPage
			});

			// 중복 제거 후 추가 (user_handle + slug 조합으로 고유성 판단)
			const existingKeys = new Set(profilePostsStore.posts.map((post) => `${post.user_handle}-${post.slug}`));
			const newPosts = response.posts.filter((post) => !existingKeys.has(`${post.user_handle}-${post.slug}`));

			profilePostsStore.addPosts(newPosts);
			profilePostsStore.setCurrentPage(response.current_page);
			profilePostsStore.setTargetPage(response.current_page); // targetPage도 업데이트
			profilePostsStore.setHasMore(response.has_more);

			console.log('Profile loadMorePosts completed: loaded', newPosts.length, 'posts from page', nextPage);
		} catch (error) {
			console.error('Failed to load more profile posts:', error);
		} finally {
			profilePostsStore.setLoading(false);
		}
	};

	// 정렬 변경 시 데이터 리로드
	const changeSortOrder = async (sortOrder: PostSortOrder) => {
		if (profilePostsStore.sortOrder === sortOrder || !profilePostsStore.userHandle) {
			return;
		}

		console.log('Profile changeSortOrder called with:', sortOrder);
		const currentUserHandle = profilePostsStore.userHandle;
		profilePostsStore.reset(currentUserHandle, sortOrder); // targetPage는 1로 리셋
		await loadInitialPosts(currentUserHandle, sortOrder);
	};

	return {
		// Store getters
		get posts() {
			return profilePostsStore.posts;
		},
		get loading() {
			return profilePostsStore.loading;
		},
		get initialLoading() {
			return profilePostsStore.initialLoading;
		},
		get hasMore() {
			return profilePostsStore.hasMore;
		},
		get currentPage() {
			return profilePostsStore.currentPage;
		},
		get sortOrder() {
			return profilePostsStore.sortOrder;
		},
		get userHandle() {
			return profilePostsStore.userHandle;
		},
		get initialized() {
			return profilePostsStore.initialized;
		},

		// Actions
		loadInitialPosts,
		loadMorePosts,
		changeSortOrder
	};
}
