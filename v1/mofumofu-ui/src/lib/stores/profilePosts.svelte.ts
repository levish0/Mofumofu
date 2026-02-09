import type { PostListItem, PostSortOrder } from '$lib/api/post/types';

interface ProfilePostsState {
	posts: PostListItem[];
	currentPage: number;
	targetPage: number;
	hasMore: boolean;
	loading: boolean;
	initialLoading: boolean;
	sortOrder: PostSortOrder;
	userHandle: string;
	initialized: boolean;
}

let state = $state<ProfilePostsState>({
	posts: [],
	currentPage: 1,
	targetPage: 1,
	hasMore: true,
	loading: false,
	initialLoading: true,
	sortOrder: 'latest',
	userHandle: '',
	initialized: false
});

export const profilePostsStore = {
	get posts() {
		return state.posts;
	},
	get currentPage() {
		return state.currentPage;
	},
	get targetPage() {
		return state.targetPage;
	},
	get hasMore() {
		return state.hasMore;
	},
	get loading() {
		return state.loading;
	},
	get initialLoading() {
		return state.initialLoading;
	},
	get sortOrder() {
		return state.sortOrder;
	},
	get userHandle() {
		return state.userHandle;
	},
	get initialized() {
		return state.initialized;
	},

	setPosts(posts: PostListItem[]) {
		state.posts = posts;
	},
	addPosts(posts: PostListItem[]) {
		state.posts = [...state.posts, ...posts];
	},
	setCurrentPage(page: number) {
		state.currentPage = page;
	},
	setTargetPage(page: number) {
		state.targetPage = page;
	},
	setHasMore(hasMore: boolean) {
		state.hasMore = hasMore;
	},
	setLoading(loading: boolean) {
		state.loading = loading;
	},
	setInitialLoading(loading: boolean) {
		state.initialLoading = loading;
	},
	setSortOrder(order: PostSortOrder) {
		state.sortOrder = order;
	},
	setUserHandle(handle: string) {
		state.userHandle = handle;
	},
	setInitialized(initialized: boolean) {
		state.initialized = initialized;
	},

	// 사용자나 정렬이 변경될 때 리셋
	reset(userHandle?: string, sortOrder?: PostSortOrder) {
		state.posts = [];
		state.currentPage = 1;
		state.targetPage = 1;
		state.hasMore = true;
		state.loading = false;
		state.initialLoading = true;
		if (userHandle) {
			state.userHandle = userHandle;
		}
		if (sortOrder) {
			state.sortOrder = sortOrder;
		}
		state.initialized = false;
	}
};
