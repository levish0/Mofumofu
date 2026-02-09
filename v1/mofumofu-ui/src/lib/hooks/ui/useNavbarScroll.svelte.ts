// src/lib/hooks/useNavbarScroll.svelte.ts
import { onMount, onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';

interface NavbarScrollOptions {
	navbarHeight?: number;
	scrollThreshold?: number;
}

export function useNavbarScroll(options: NavbarScrollOptions = {}): {
	isVisible: () => boolean;
	isAtTop: () => boolean;
} {
	const { navbarHeight = 60, scrollThreshold = 10 } = options;

	// 초기값을 false로 설정하여 깜빡임 방지
	let isVisible = $state(false);
	let isAtTop = $state(true); // 맨 위에 있는지 여부
	let lastScrollY = $state(0);
	let rafId: number | null = null;

	const handleScroll = () => {
		if (rafId) return;

		rafId = requestAnimationFrame(() => {
			const currentScrollY = window.scrollY;
			const scrollDelta = Math.abs(currentScrollY - lastScrollY);

			// 스크롤 변화가 임계값보다 작으면 무시
			if (scrollDelta < scrollThreshold) {
				rafId = null;
				return;
			}

			// 맨 위에 있는지 확인
			isAtTop = currentScrollY <= navbarHeight;

			// 맨 위에 있으면 항상 보이기
			if (currentScrollY <= navbarHeight) {
				isVisible = true;
			} else {
				// 스크롤 방향에 따라 가시성 결정
				isVisible = currentScrollY < lastScrollY;
			}

			lastScrollY = currentScrollY;
			rafId = null;
		});
	};

	onMount(() => {
		if (!BROWSER) return;

		// 초기 스크롤 위치 설정 및 초기 가시성 결정
		const currentScrollY = window.scrollY;
		lastScrollY = currentScrollY;

		// 초기 가시성 상태 설정
		isVisible = currentScrollY <= navbarHeight;
		isAtTop = currentScrollY <= navbarHeight;

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

	return {
		isVisible: () => isVisible,
		isAtTop: () => isAtTop
	};
}
