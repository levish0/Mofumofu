<script module lang="ts">
	export const PILL_BUTTON_CLASS = 'rounded-full font-bold';

	export const PILL_VARIANT_CLASSES = {
		default: 'bg-black text-white hover:opacity-75 dark:bg-white dark:text-black',
		outline:
			'border-2 border-black bg-transparent text-black hover:bg-black hover:text-white dark:border-white dark:text-white dark:hover:bg-white dark:hover:text-black'
	} as const;
</script>

<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		variant = 'default',
		class: className,
		...rest
	}: ComponentProps<typeof Button> = $props();

	const pillVariant = $derived(
		variant === 'outline' || variant === 'default' ? PILL_VARIANT_CLASSES[variant] : ''
	);
</script>

<Button bind:ref {variant} class={cn(PILL_BUTTON_CLASS, pillVariant, className)} {...rest} />
