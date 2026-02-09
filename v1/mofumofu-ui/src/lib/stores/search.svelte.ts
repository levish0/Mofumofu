import type { PostListItem } from '$lib/api/post/types';

class SearchStore {
	private _searchQuery = $state('');
	private _posts = $state<PostListItem[]>([]);
	private _loading = $state(false);
	private _hasMore = $state(false);
	private _currentPage = $state(1);
	private _totalResults = $state(0);
	private _hasSearched = $state(false);

	// Getters
	get searchQuery() {
		return this._searchQuery;
	}

	get posts() {
		return this._posts;
	}

	get loading() {
		return this._loading;
	}

	get hasMore() {
		return this._hasMore;
	}

	get currentPage() {
		return this._currentPage;
	}

	get totalResults() {
		return this._totalResults;
	}

	get hasSearched() {
		return this._hasSearched;
	}

	// Setters
	setSearchQuery(query: string) {
		this._searchQuery = query;
	}

	setPosts(posts: PostListItem[]) {
		this._posts = posts;
	}

	addPosts(newPosts: PostListItem[]) {
		this._posts = [...this._posts, ...newPosts];
	}

	setLoading(loading: boolean) {
		this._loading = loading;
	}

	setHasMore(hasMore: boolean) {
		this._hasMore = hasMore;
	}

	setCurrentPage(page: number) {
		this._currentPage = page;
	}

	setTotalResults(total: number) {
		this._totalResults = total;
	}

	setHasSearched(hasSearched: boolean) {
		this._hasSearched = hasSearched;
	}

	// Reset search state
	reset() {
		this._posts = [];
		this._loading = false;
		this._hasMore = false;
		this._currentPage = 1;
		this._totalResults = 0;
		this._hasSearched = false;
	}

	// Clear everything including search query
	clear() {
		this._searchQuery = '';
		this.reset();
	}
}

export const searchStore = new SearchStore();
