<script lang="ts">
	import type { HTMLInputAttributes, HTMLInputTypeAttribute } from 'svelte/elements';
	import { cn, type WithElementRef } from '$lib/utils/index.js';

	type InputType = Exclude<HTMLInputTypeAttribute, 'file'>;

	type Props = WithElementRef<
		Omit<HTMLInputAttributes, 'type'> & ({ type: 'file'; files?: FileList } | { type?: InputType; files?: undefined })
	>;

	let {
		ref = $bindable(null),
		value = $bindable(),
		type,
		files = $bindable(),
		class: className,
		...restProps
	}: Props = $props();
</script>

{#if type === 'file'}
	<input
		bind:this={ref}
		data-slot="input"
		class={cn(
			' ring-offset-background placeholder:text-muted-foreground flex h-9 w-full min-w-0 rounded-md  bg-white/70 px-3  pt-1.5 text-sm font-medium  transition-[color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-black',
			'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
			className
		)}
		type="file"
		bind:files
		bind:value
		{...restProps}
	/>
{:else}
	<input
		bind:this={ref}
		data-slot="input"
		class={cn(
			'ring-offset-background placeholder:text-muted-foreground  flex h-9 w-full min-w-0 rounded-md bg-white/70 px-3  py-1 text-base  transition-[color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-black',
			'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
			className
		)}
		{type}
		bind:value
		{...restProps}
	/>
{/if}
