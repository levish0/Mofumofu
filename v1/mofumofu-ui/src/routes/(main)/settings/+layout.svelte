<script lang="ts">
	let { children } = $props();
</script>

<div class="max-w-8xl mx-auto pb-40">
	<div class="px-4 pt-2">{@render children()}</div>
</div>
