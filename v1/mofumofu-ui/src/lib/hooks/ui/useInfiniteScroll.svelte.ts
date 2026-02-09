import { onMount, onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';

interface InfiniteScrollOptions {
	threshold?: number;
	onLoadMore: () => Promise<void>;
	isLoading?: () => boolean;
	hasMore?: () => boolean;
}

export function useInfiniteScroll(options: InfiniteScrollOptions) {
	const { threshold = 200, onLoadMore, isLoading = () => false, hasMore = () => true } = options;

	let rafId: number | null = null;

	const handleScroll = () => {
		if (rafId) return;

		rafId = requestAnimationFrame(async () => {
			// 이미 로딩 중이거나 더 이상 로드할 데이터가 없으면 무시
			if (isLoading() || !hasMore()) {
				rafId = null;
				return;
			}

			const { scrollTop, scrollHeight, clientHeight } = document.documentElement;

			// 하단에서 threshold만큼 떨어진 지점에 도달하면 로드
			if (scrollTop + clientHeight >= scrollHeight - threshold) {
				try {
					await onLoadMore();
				} catch (error) {
					console.error('Failed to load more content:', error);
				}
			}

			rafId = null;
		});
	};

	onMount(() => {
		if (!BROWSER) return;

		// 스크롤 이벤트 등록
		window.addEventListener('scroll', handleScroll, { passive: true });
	});

	onDestroy(() => {
		if (!BROWSER) return;

		window.removeEventListener('scroll', handleScroll);

		if (rafId) {
			cancelAnimationFrame(rafId);
		}
	});
}
