import { onMount } from 'svelte';

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

		if (lineCount <= lineThreshold) {
			showStickyToolbar = false;
			return;
		}

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
		const initialLineCount = textareaElement.value.split('\n').length;
		showStickyToolbar = initialLineCount > lineThreshold;

		textareaElement.addEventListener('input', updateToolbarVisibility, { passive: true });
		textareaElement.addEventListener('scroll', handleScroll, { passive: true });

		return () => {
			textareaElement.removeEventListener('input', updateToolbarVisibility);
			textareaElement.removeEventListener('scroll', handleScroll);
			if (rafId) cancelAnimationFrame(rafId);
		};
	});

	return {
		showStickyToolbar: () => showStickyToolbar
	};
}
