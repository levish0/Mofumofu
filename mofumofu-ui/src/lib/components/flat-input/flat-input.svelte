<script module lang="ts">
	export const FLAT_INPUT_CLASS =
		'border-0 shadow-none rounded-lg bg-white/70 dark:bg-white/[0.06]';
</script>

<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { Input } from '$lib/components/ui/input';
	import { cn } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		...rest
	}: ComponentProps<typeof Input> = $props();
</script>

<Input bind:ref bind:value class={cn(FLAT_INPUT_CLASS, className)} {...rest} />
