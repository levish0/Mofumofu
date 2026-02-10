import { onMount } from 'svelte';

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

		const containerWidth = container.getBoundingClientRect().width;
		const deltaX = e.clientX - startX;
		const deltaRatio = deltaX / containerWidth;
		leftRatio = Math.min(maxRatio, Math.max(minRatio, startRatio + deltaRatio));
	};

	const handleMouseUp = () => {
		isDragging = false;
	};

	onMount(() => {
		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', handleMouseUp);

		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', handleMouseUp);
		};
	});

	return {
		leftRatio: () => leftRatio,
		isDragging: () => isDragging,
		handleMouseDown
	};
}
