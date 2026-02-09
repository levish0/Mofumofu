<script lang="ts">
	import { Pagination as PaginationPrimitive } from 'bits-ui';
	import { cn, type WithElementRef } from '$lib/utils/index';
	import { type Props, buttonVariants } from '$lib/components/ui/button/index.js';

	let {
		ref = $bindable(null),
		class: className,
		isActive,
		page,
		children,
		...restProps
	}: PaginationPrimitive.PageProps &
		Props & {
			isActive: boolean;
		} = $props();
</script>

{#snippet Fallback()}
	{page.value}
{/snippet}

<PaginationPrimitive.Page
	bind:ref
	{page}
	aria-current={isActive ? 'page' : undefined}
	data-slot="pagination-link"
	data-active={isActive}
	class={cn(
		buttonVariants({
			variant: isActive ? 'outline' : 'ghost'
		}),
		className
	)}
	children={children || Fallback}
	{...restProps}
/>
