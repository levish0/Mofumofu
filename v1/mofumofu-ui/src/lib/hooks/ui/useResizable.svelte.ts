// src/lib/hooks/useResizable.svelte.ts
import { onMount, onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';

interface ResizableOptions {
	minWidth?: number;
	maxWidthPercent?: number;
	initialWidth?: number;
}

export function useResizable(
	containerElement: HTMLElement | null,
	options: ResizableOptions = {}
): {
	leftWidth: () => number;
	rightWidth: () => number;
	isDragging: () => boolean;
	handleMouseDown: (e: MouseEvent) => void;
} {
	const { minWidth = 400, maxWidthPercent = 80, initialWidth = 50 } = options;

	let leftWidth = $state(initialWidth); // percentage
	let isDragging = $state(false);
	let startX = 0;
	let startWidth = 0;

	const handleMouseDown = (e: MouseEvent) => {
		if (!containerElement) return;

		isDragging = true;
		startX = e.clientX;
		startWidth = leftWidth;

		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
		document.body.style.cursor = 'col-resize';
		document.body.style.userSelect = 'none';
	};

	const handleMouseMove = (e: MouseEvent) => {
		if (!isDragging || !containerElement) return;

		const containerRect = containerElement.getBoundingClientRect();
		const deltaX = e.clientX - startX;
		const deltaPercent = (deltaX / containerRect.width) * 100;

		let newWidth = startWidth + deltaPercent;

		// 최소/최대 너비 제한
		const minPercent = (minWidth / containerRect.width) * 100;
		const maxPercent = maxWidthPercent;

		newWidth = Math.max(minPercent, Math.min(maxPercent, newWidth));
		leftWidth = newWidth;
	};

	const handleMouseUp = () => {
		isDragging = false;
		document.removeEventListener('mousemove', handleMouseMove);
		document.removeEventListener('mouseup', handleMouseUp);
		document.body.style.cursor = '';
		document.body.style.userSelect = '';
	};

	onMount(() => {
		if (!BROWSER) return;
		// 초기화는 컴포넌트에서 핸들러를 연결할 때 처리
	});

	onDestroy(() => {
		if (!BROWSER) return;
		document.removeEventListener('mousemove', handleMouseMove);
		document.removeEventListener('mouseup', handleMouseUp);
		document.body.style.cursor = '';
		document.body.style.userSelect = '';
	});

	return {
		leftWidth: () => leftWidth,
		rightWidth: () => 100 - leftWidth,
		isDragging: () => isDragging,
		handleMouseDown
	};
}
