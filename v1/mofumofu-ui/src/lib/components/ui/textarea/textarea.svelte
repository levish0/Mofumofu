<script lang="ts">
	import { cn, type WithElementRef, type WithoutChildren } from '$lib/utils/index.js';
	import type { HTMLTextareaAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		...restProps
	}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> = $props();
</script>

<textarea
	bind:this={ref}
	data-slot="textarea"
	class={cn(
		' placeholder:text-muted-foreground  aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40  dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md  bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none  disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
		className
	)}
	bind:value
	{...restProps}
></textarea>
