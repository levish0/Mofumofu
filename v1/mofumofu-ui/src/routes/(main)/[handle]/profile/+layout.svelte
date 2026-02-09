<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
</script>

<div class="min-h-screen pb-20">
	{@render children()}
</div>
