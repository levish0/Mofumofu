import { onMount, onDestroy } from 'svelte';

interface ResizableOptions {
	initialRatio?: number;
	minRatio?: number;
	maxRatio?: number;
}

export function useResizable(options: ResizableOptions = {}): {
	leftRatio: () => number;
	isDragging: () => boolean;
	handleMouseDown: (e: MouseEvent) => void;
} {
	const { initialRatio = 0.5, minRatio = 0.25, maxRatio = 0.75 } = options;

	let leftRatio = $state(initialRatio);
	let isDragging = $state(false);
	let containerWidth = 0;
	let startX = 0;
	let startRatio = 0;

	const handleMouseDown = (e: MouseEvent) => {
		isDragging = true;
		startX = e.clientX;
		startRatio = leftRatio;
		e.preventDefault();
	};

	const handleMouseMove = (e: MouseEvent) => {
		if (!isDragging) return;

		const container = document.getElementById('resizable-container');
		if (!container) return;

		containerWidth = container.getBoundingClientRect().width;
		const deltaX = e.clientX - startX;
		const deltaRatio = deltaX / containerWidth;
		const newRatio = Math.min(maxRatio, Math.max(minRatio, startRatio + deltaRatio));
		leftRatio = newRatio;
	};

	const handleMouseUp = () => {
		isDragging = false;
	};

	onMount(() => {
		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', handleMouseUp);
	});

	onDestroy(() => {
		window.removeEventListener('mousemove', handleMouseMove);
		window.removeEventListener('mouseup', handleMouseUp);
	});

	return {
		leftRatio: () => leftRatio,
		isDragging: () => isDragging,
		handleMouseDown
	};
}
