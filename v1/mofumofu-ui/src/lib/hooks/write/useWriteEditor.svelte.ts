// src/lib/hooks/useWriteEditor.svelte.ts
import { onMount, onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';

interface WriteEditorOptions {
	lineThreshold?: number;
}

export function useWriteEditor(
	textareaElement: HTMLTextAreaElement,
	options: WriteEditorOptions = {}
): {
	showStickyToolbar: () => boolean;
} {
	const { lineThreshold = 30 } = options;

	let showStickyToolbar = $state(false);
	let rafId: number | null = null;
	let lastScrollTop = 0;
	let isScrollingUp = false;

	const updateToolbarVisibility = () => {
		const content = textareaElement.value;
		const lineCount = content.split('\n').length;

		// 30줄 넘지 않으면 항상 보여줌
		if (lineCount <= lineThreshold) {
			showStickyToolbar = false;
			return;
		}

		// 스크롤이 위에 있으면 보여줌, 아래에 있으면 숨김
		showStickyToolbar = !isScrollingUp;
	};

	const handleScroll = () => {
		if (rafId) return;

		rafId = requestAnimationFrame(() => {
			const currentScrollTop = textareaElement.scrollTop;
			isScrollingUp = currentScrollTop < lastScrollTop;
			lastScrollTop = currentScrollTop;

			updateToolbarVisibility();
			rafId = null;
		});
	};

	onMount(() => {
		if (!BROWSER) return;

		// 초기 상태 설정
		const initialLineCount = textareaElement.value.split('\n').length;
		showStickyToolbar = initialLineCount > lineThreshold;

		// 이벤트 리스너 등록
		textareaElement.addEventListener('input', updateToolbarVisibility, { passive: true });
		textareaElement.addEventListener('scroll', handleScroll, { passive: true });
	});

	onDestroy(() => {
		if (!BROWSER) return;

		textareaElement.removeEventListener('input', updateToolbarVisibility);
		textareaElement.removeEventListener('scroll', handleScroll);

		if (rafId) {
			cancelAnimationFrame(rafId);
		}
	});

	return {
		showStickyToolbar: () => showStickyToolbar
	};
}
