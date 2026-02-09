<script lang="ts" module>
	import { cn, type WithElementRef } from '$lib/utils/index.js';
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';
	import { type VariantProps, tv } from 'tailwind-variants';

	export const buttonVariants = tv({
		base: 'px-4 py-2 h-9 inline-flex items-center justify-center whitespace-nowrap rounded-full font-bold outline-none transition-all disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0',
		variants: {
			variant: {
				default: 'dark:bg-white dark:text-black text-white bg-black hover:opacity-75 ',
				destructive: 'bg-rose-600 text-white',
				outline:
					'bg-transparent border-black hover:bg-black hover:text-white text-black border-2 dark:border-white dark:hover:bg-white dark:hover:text-black dark:text-white',
				ghost: 'hover:opacity-70 text-black dark:text-white',
				link: 'text-black dark:text-white underline-offset-4 hover:underline',
				icon: 'h-9 w-9 p-2 hover:bg-white/10 text-white'
			}
		},
		defaultVariants: {
			variant: 'default'
		}
	});

	export type ButtonVariant = VariantProps<typeof buttonVariants>['variant'];

	export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
		WithElementRef<HTMLAnchorAttributes> & {
			variant?: ButtonVariant;
		};
</script>

<script lang="ts">
	let {
		class: className,
		variant = 'default',
		ref = $bindable(null),
		href = undefined,
		type = 'button',
		disabled,
		children,
		...restProps
	}: ButtonProps = $props();
</script>

{#if href}
	<a
		bind:this={ref}
		data-slot="button"
		class={cn(buttonVariants({ variant }), className)}
		href={disabled ? undefined : href}
		aria-disabled={disabled}
		role={disabled ? 'link' : undefined}
		tabindex={disabled ? -1 : undefined}
		{...restProps}
	>
		{@render children?.()}
	</a>
{:else}
	<button
		bind:this={ref}
		data-slot="button"
		class={cn(buttonVariants({ variant }), className)}
		{type}
		{disabled}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}
