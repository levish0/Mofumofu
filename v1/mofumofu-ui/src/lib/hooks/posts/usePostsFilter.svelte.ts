// src/lib/hooks/usePostsFilter.svelte.ts

import { postsStore } from '$lib/stores/posts.svelte';

interface UsePostsFilterConfig {
	onFilterChange: () => void;
	debounceMs?: number;
}

export function usePostsFilter(config: UsePostsFilterConfig) {
	const { onFilterChange, debounceMs = 500 } = config;

	const filter = $derived(postsStore.filter);
	const initialLoading = $derived(postsStore.initialLoading);

	// 이전 값들 추적 - Store의 기본값과 동일하게 초기화
	let prevSortBy = $state('recent');
	let prevTimeRange = $state('all');
	let searchTimeout: number | null = null;

	// 정렬 변경 감지 (즉시 반영)
	$effect(() => {
		const currentSortBy = filter.sortBy;
		const currentTimeRange = filter.timeRange;

		if ((currentSortBy !== prevSortBy || currentTimeRange !== prevTimeRange) && !initialLoading) {
			prevSortBy = currentSortBy;
			prevTimeRange = currentTimeRange;
			onFilterChange();
		}
	});

	// TODO: 검색 기능 제거로 인한 주석 처리
	// 검색어/태그 변경 감지 (debounced)
	// $effect(() => {
	// 	const currentKeyword = filter.keyword;
	// 	const currentTags = filter.tags;
	// 	const keywordChanged = currentKeyword !== prevKeyword;
	// 	const tagsChanged = JSON.stringify(currentTags) !== JSON.stringify(prevTags);

	// 	if ((keywordChanged || tagsChanged) && !initialLoading) {
	// 		prevKeyword = currentKeyword;
	// 		prevTags = [...currentTags];

	// 		// 기존 timeout 취소
	// 		if (searchTimeout) {
	// 			clearTimeout(searchTimeout);
	// 		}

	// 		// 검색어가 있으면 debounce, 없으면 즉시
	// 		if (currentKeyword || currentTags.length > 0) {
	// 			searchTimeout = window.setTimeout(() => {
	// 				onFilterChange();
	// 			}, debounceMs);
	// 		} else {
	// 			// 검색어와 태그가 모두 비어있으면 즉시 일반 모드로
	// 			onFilterChange();
	// 		}
	// 	}
	// });

	// cleanup
	return () => {
		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}
	};
}
