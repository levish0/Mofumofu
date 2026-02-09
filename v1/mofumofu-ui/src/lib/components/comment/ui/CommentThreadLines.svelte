<script lang="ts">
	import { ChevronDown, ChevronUp } from '@lucide/svelte';

	interface Props {
		depth: number;
		isLast: boolean;
		hasReplies: boolean;
		showChildren: boolean;
		onToggleChildren: () => void;
	}

	let { depth, isLast, hasReplies, showChildren, onToggleChildren }: Props = $props();
</script>

{#if isLast}
	<div class="absolute -left-3 h-full">
		<div class="bg-mofu-light-950 dark:bg-mofu-dark-900 h-full w-6"></div>
	</div>
	<div
		class="border-mofu-light-300 dark:border-mofu-dark-600 absolute left-[9px] h-full w-4 border-l border-dashed"
	></div>
	<div class="bg-mofu-light-950 dark:bg-mofu-dark-900 absolute -bottom-4 -left-1 h-4 w-4"></div>
{/if}

{#if depth > 0}
	<div
		class="border-mofu-light-300 dark:border-mofu-dark-600 absolute left-[9px] h-7 w-4 rounded-bl-full border-b border-l"
	></div>
{/if}

{#if hasReplies}
	<div class="pointer-events-none absolute top-0 left-[41px] h-full w-px pt-12 pb-6">
		<div class="border-mofu-light-300 dark:border-mofu-dark-600 h-full w-full border-l"></div>
		<button
			onclick={onToggleChildren}
			class="group/btn bg-mofu-light-950 dark:bg-mofu-dark-900 border-mofu-light-300 dark:border-mofu-dark-600 pointer-events-auto absolute top-14 -left-[9.5px] z-20 cursor-pointer rounded-full border p-0.5"
			title={showChildren ? '답글 접기' : '답글 펼치기'}
			aria-label={showChildren ? '답글 접기' : '답글 펼치기'}
		>
			{#if showChildren}
				<ChevronUp class="size-3.5 opacity-60 transition-opacity group-hover/btn:opacity-100" strokeWidth={2} />
			{:else}
				<ChevronDown class="size-3.5 opacity-60 transition-opacity group-hover/btn:opacity-100" strokeWidth={2} />
			{/if}
		</button>
	</div>
{/if}
