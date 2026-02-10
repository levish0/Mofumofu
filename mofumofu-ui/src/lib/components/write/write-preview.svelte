<script lang="ts">
	import '$lib/styles/markdown.css';
	import { onMount } from 'svelte';
	import { mode } from 'mode-watcher';

	interface Props {
		title: string;
		htmlOutput: string;
	}

	const { title, htmlOutput }: Props = $props();

	function updateHighlightTheme(isDark: boolean) {
		if (typeof document === 'undefined') return;

		const existing = document.querySelector('link[data-highlight-theme]');
		if (existing) existing.remove();

		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = isDark
			? 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/night-owl.css'
			: 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/atom-one-light.css';
		link.setAttribute('data-highlight-theme', isDark ? 'dark' : 'light');
		document.head.appendChild(link);
	}

	$effect(() => {
		updateHighlightTheme(mode.current === 'dark');
	});

	onMount(() => {
		updateHighlightTheme(mode.current === 'dark');
	});
</script>

<div class="flex h-full flex-col bg-background pt-20 pl-8">
	<div class="mb-4">
		<h1 class="text-4xl font-bold" style="font-size: 2.5rem; line-height: 1.2;">
			{title}
		</h1>
	</div>

	<div class="flex-1 overflow-auto pt-4 pr-4 pb-4">
		<div class="prose prose-lg dark:prose-invert max-w-none">
			{@html htmlOutput}
		</div>
	</div>
</div>
