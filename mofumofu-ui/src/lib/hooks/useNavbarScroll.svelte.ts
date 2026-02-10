interface NavbarScrollOptions {
	navbarHeight?: number;
	scrollThreshold?: number;
}

export function useNavbarScroll(options: NavbarScrollOptions = {}): {
	isVisible: () => boolean;
	isAtTop: () => boolean;
} {
	const { navbarHeight = 60, scrollThreshold = 10 } = options;

	let isVisible = $state(true);
	let isAtTop = $state(true);
	let lastScrollY = 0;
	let rafId: number | null = null;

	const handleScroll = () => {
		if (rafId) return;

		rafId = requestAnimationFrame(() => {
			const currentScrollY = window.scrollY;
			const scrollDelta = Math.abs(currentScrollY - lastScrollY);

			if (scrollDelta < scrollThreshold) {
				rafId = null;
				return;
			}

			isAtTop = currentScrollY <= navbarHeight;
			isVisible = currentScrollY <= navbarHeight || currentScrollY < lastScrollY;

			lastScrollY = currentScrollY;
			rafId = null;
		});
	};

	$effect(() => {
		lastScrollY = window.scrollY;
		isVisible = lastScrollY <= navbarHeight;
		isAtTop = lastScrollY <= navbarHeight;

		window.addEventListener('scroll', handleScroll, { passive: true });

		return () => {
			window.removeEventListener('scroll', handleScroll);
			if (rafId) cancelAnimationFrame(rafId);
		};
	});

	return {
		isVisible: () => isVisible,
		isAtTop: () => isAtTop
	};
}
