<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
</script>

<main class="px-4 pt-2">
	{@render children()}
</main>
