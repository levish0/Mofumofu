<script lang="ts">
	import './layout.css';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	let { children } = $props();
</script>

<svelte:head>
	<meta
		name="keywords"
		content="blogging, writing, essays, tutorials, minimalist, open-source, blog platform"
	/>
	<meta name="robots" content="index, follow" />
	<meta property="og:site_name" content="Mofumofu" />
</svelte:head>

<ModeWatcher defaultMode="dark" />
<Toaster />
<div class=" min-h-screen max-w-screen bg-mofu-light-900 font-pretendard dark:bg-mofu-dark-900">
	<main>
		{@render children()}
	</main>
</div>
