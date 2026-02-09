<script lang="ts">
	import { onMount } from 'svelte';
	import { mode } from 'mode-watcher';

	interface Props {
		title: string;
		htmlOutput: string;
	}

	const { title, htmlOutput }: Props = $props();

	function updateHighlightTheme(isDark: boolean) {
		if (typeof document === 'undefined') return;

		// Remove existing highlight.js theme
		const existingLink = document.querySelector('link[data-highlight-theme]');
		if (existingLink) {
			existingLink.remove();
		}

		// Add new theme
		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = isDark
			? 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/night-owl.css'
			: 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/atom-one-light.css';
		link.setAttribute('data-highlight-theme', isDark ? 'dark' : 'light');

		document.head.appendChild(link);
	}

	// Watch for theme changes
	$effect(() => {
		updateHighlightTheme(mode.current === 'dark');
	});

	onMount(() => {
		updateHighlightTheme(mode.current === 'dark');
	});
</script>

<div
	class="dark:bg-mofu-dark-950 bg-mofu-light-950 text-mofu-light-200 dark:text-mofu-dark-200 flex h-full flex-col pt-20 pl-8"
>
	<div class="mb-4">
		<h1 class="text-4xl font-bold" style="font-size: 2.5rem; line-height: 1.2;">
			{title}
		</h1>
	</div>

	<div class="flex-1 overflow-auto pt-4 pr-4 pb-4">
		<div class="prose prose-invert dark:text-mofu-dark-200 text-mofu-light-200 prose-lg max-w-none">
			{@html htmlOutput}
		</div>
	</div>
</div>
